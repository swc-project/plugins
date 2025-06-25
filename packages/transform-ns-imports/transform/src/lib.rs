use std::{borrow::Cow, collections::HashMap, sync::Arc};

use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use serde::Deserialize;
use swc_atoms::Atom;
use swc_cached::regex::CachedRegex;
use swc_common::{util::take::Take, DUMMY_SP};
use swc_ecma_ast::{ImportDecl, ImportSpecifier, ModuleExportName, *};
use swc_ecma_visit::{noop_visit_mut_type, visit_mut_pass, VisitMut, VisitMutWith};

static DUP_SLASH_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"//").unwrap());

#[derive(Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct Config {
    pub packages: HashMap<String, Arc<PackageConfig>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageConfig {
    #[serde(default)]
    pub rewrite: String,
}

struct TransformImports<'a> {
    packages: Vec<(CachedRegex, &'a PackageConfig)>,
}

struct Rewriter<'a> {
    key: &'a str,
    config: &'a PackageConfig,
    group: Vec<&'a str>,
}

impl Rewriter<'_> {
    fn new_path(&self) -> Atom {
        let new_path = DUP_SLASH_REGEX.replace_all(&self.config.rewrite, |_: &Captures| "/");

        new_path.into()
    }

    fn rewrite_export(&self, old_decl: &NamedExport) -> Vec<NamedExport> {
        if old_decl.type_only || old_decl.with.is_some() {
            return vec![old_decl.clone()];
        }

        let mut out = Vec::with_capacity(old_decl.specifiers.len());

        for spec in &old_decl.specifiers {
            match spec {
                ExportSpecifier::Namespace(ns_spec) => {
                    let (new_path, specifier) = (
                        self.new_path(),
                        ExportSpecifier::Named(ExportNamedSpecifier {
                            span: DUMMY_SP,
                            orig: ModuleExportName::Ident(Ident::from("default")),
                            exported: Some(ns_spec.name.clone()),
                            is_type_only: false,
                        }),
                    );

                    out.push(NamedExport {
                        specifiers: vec![specifier],
                        src: Some(Box::new(Str::from(new_path.as_ref()))),
                        span: old_decl.span,
                        type_only: false,
                        with: None,
                    });
                }
                _ => {
                    return vec![old_decl.clone()];
                }
            }
        }
        out
    }

    fn rewrite_import(&self, old_decl: &ImportDecl) -> Vec<ImportDecl> {
        if old_decl.type_only || old_decl.with.is_some() {
            return vec![old_decl.clone()];
        }

        let mut out: Vec<ImportDecl> = Vec::with_capacity(old_decl.specifiers.len());

        for spec in &old_decl.specifiers {
            match spec {
                ImportSpecifier::Namespace(ns_spec) => {
                    let (new_path, specifier) = (
                        self.new_path(),
                        ImportSpecifier::Default(ImportDefaultSpecifier {
                            span: DUMMY_SP,
                            local: ns_spec.local.clone(),
                        }),
                    );
                    out.push(ImportDecl {
                        specifiers: vec![specifier],
                        src: Box::new(Str::from(new_path.as_ref())),
                        span: old_decl.span,
                        type_only: false,
                        with: None,
                        phase: Default::default(),
                    });
                }
                _ => {
                    return vec![old_decl.clone()];
                }
            }
        }
        out
    }
}

impl TransformImports<'_> {
    fn should_rewrite<'a>(&'a self, name: &'a str) -> Option<Rewriter<'a>> {
        for (regex, config) in &self.packages {
            let group = regex.captures(name);
            if let Some(group) = group {
                let group = group
                    .iter()
                    .map(|x| x.map(|x| x.as_str()).unwrap_or_default())
                    .collect::<Vec<&str>>();
                return Some(Rewriter {
                    key: name,
                    config,
                    group,
                });
            }
        }
        None
    }
}

impl VisitMut for TransformImports<'_> {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, module: &mut Module) {
        module.visit_mut_children_with(self);

        let mut new_items: Vec<ModuleItem> = Vec::with_capacity(module.body.len());
        for item in module.body.take() {
            match item {
                ModuleItem::ModuleDecl(ModuleDecl::Import(decl)) => {
                    if decl.specifiers.is_empty() {
                        if let Some(rewriter) = self.should_rewrite(&decl.src.value) {
                            let new_path = rewriter.new_path();
                            let raw_with_quotes = Atom::from(format!("'{}'", new_path.as_ref()));
                            let new_src = Box::new(Str {
                                span: decl.src.span,
                                value: new_path.clone(),
                                raw: Some(raw_with_quotes),
                            });
                            let new_decl = ImportDecl {
                                src: new_src,
                                specifiers: vec![],
                                ..decl
                            };

                            new_items.push(ModuleItem::ModuleDecl(ModuleDecl::Import(new_decl)));
                        } else {
                            new_items.push(ModuleItem::ModuleDecl(ModuleDecl::Import(decl)));
                        }
                        continue;
                    }

                    match self.should_rewrite(&decl.src.value) {
                        Some(rewriter) => {
                            let rewritten = rewriter.rewrite_import(&decl);
                            new_items.extend(
                                rewritten
                                    .into_iter()
                                    .map(ModuleDecl::Import)
                                    .map(ModuleItem::ModuleDecl),
                            );
                        }
                        None => new_items.push(ModuleItem::ModuleDecl(ModuleDecl::Import(decl))),
                    }
                }
                ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(
                    decl @ NamedExport { src: Some(..), .. },
                )) => match self.should_rewrite(&decl.src.as_deref().unwrap().value) {
                    Some(rewriter) => {
                        let rewritten = rewriter.rewrite_export(&decl);
                        new_items.extend(
                            rewritten
                                .into_iter()
                                .map(ModuleDecl::ExportNamed)
                                .map(ModuleItem::ModuleDecl),
                        );
                    }
                    None => new_items.push(ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(decl))),
                },

                ModuleItem::ModuleDecl(ModuleDecl::ExportAll(e @ ExportAll { .. })) => {
                    match self.should_rewrite(&e.src.value) {
                        Some(rewriter) => {
                            let rewritten = rewriter.new_path();
                            new_items.push(ModuleItem::ModuleDecl(ModuleDecl::ExportAll(
                                ExportAll {
                                    src: Box::new(Str {
                                        span: e.src.span,
                                        value: rewritten,
                                        raw: None,
                                    }),
                                    ..e
                                },
                            )));
                        }
                        None => {
                            new_items.push(ModuleItem::ModuleDecl(ModuleDecl::ExportAll(e)));
                        }
                    }
                }
                _ => {
                    new_items.push(item);
                }
            }
        }
        module.body = new_items;
    }
}

pub fn transform_ns_imports(config: &Config) -> impl '_ + Pass {
    let mut folder = TransformImports { packages: vec![] };

    for (k, v) in &config.packages {
        let mut k = Cow::Borrowed(k);
        // XXX: Should we keep this hack?
        if !k.starts_with('^') && !k.ends_with('$') {
            k = Cow::Owned(format!("^{k}$"));
        }
        folder.packages.push((
            CachedRegex::new(&k).expect("transform-ns-imports: invalid regex"),
            v,
        ));
    }
    visit_mut_pass(folder)
}

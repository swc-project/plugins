use std::collections::HashMap;

use convert_case::{Case, Casing};
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use swc_atoms::Atom;
use swc_cached::regex::CachedRegex;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{ImportDecl, ImportSpecifier, ModuleExportName, *};
use swc_ecma_visit::{fold_pass, noop_fold_type, Fold, FoldWith};

static DUP_SLASH_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"//").unwrap());

#[derive(Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct Config {
    pub packages: HashMap<String, PackageConfig>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageConfig {
    pub transform: Transform,
    #[serde(default)]
    pub prevent_full_import: bool,
    #[serde(default)]
    pub handle_default_import: bool,
    #[serde(default)]
    pub handle_namespace_import: bool,
    #[serde(default)]
    pub skip_default_conversion: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Transform {
    String(String),
    Vec(Vec<(String, String)>),
}

impl From<&str> for Transform {
    fn from(s: &str) -> Self {
        Transform::String(s.to_string())
    }
}
impl From<Vec<(String, String)>> for Transform {
    fn from(v: Vec<(String, String)>) -> Self {
        Transform::Vec(v)
    }
}

struct FoldImports {
    renderer: handlebars::Handlebars<'static>,
    packages: Vec<(CachedRegex, PackageConfig)>,
}

struct Rewriter<'a> {
    renderer: &'a handlebars::Handlebars<'static>,
    key: &'a str,
    config: &'a PackageConfig,
    group: Vec<&'a str>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum CtxData<'a> {
    Plain(&'a str),
    Array(&'a [&'a str]),
}

impl Rewriter<'_> {
    fn new_path(&self, name_str: Option<&str>) -> Atom {
        let mut ctx: HashMap<&str, CtxData> = HashMap::new();
        ctx.insert("matches", CtxData::Array(&self.group[..]));
        if let Some(name_str) = name_str {
            ctx.insert("member", CtxData::Plain(name_str));
        }

        let new_path = match &self.config.transform {
            Transform::String(s) => self.renderer.render_template(s, &ctx).unwrap_or_else(|e| {
                panic!("error rendering template for '{}': {}", self.key, e);
            }),
            Transform::Vec(v) => {
                let mut result: Option<String> = None;

                // We iterate over the items to find the first match
                v.iter().any(|(k, val)| {
                    let mut key = k.to_string();
                    if !key.starts_with('^') && !key.ends_with('$') {
                        key = format!("^{}$", key);
                    }

                    // Create a clone of the context, as we need to insert the
                    // `memberMatches` key for each key we try.
                    let mut ctx_with_member_matches: HashMap<&str, CtxData> = HashMap::new();
                    ctx_with_member_matches.insert("matches", CtxData::Array(&self.group[..]));

                    if let Some(name_str) = name_str {
                        ctx_with_member_matches.insert("member", CtxData::Plain(name_str));
                    }
                    let regex = CachedRegex::new(&key).expect("transform-imports: invalid regex");
                    if let Some(name_str) = name_str {
                        let group = regex.captures(name_str);

                        if let Some(group) = group {
                            let group = group
                                .iter()
                                .map(|x| x.map(|x| x.as_str()).unwrap_or_default())
                                .collect::<Vec<&str>>()
                                .clone();
                            ctx_with_member_matches
                                .insert("memberMatches", CtxData::Array(&group[..]));

                            result = Some(
                                self.renderer
                                    .render_template(val, &ctx_with_member_matches)
                                    .unwrap_or_else(|e| {
                                        panic!(
                                            "error rendering template for '{}': {}",
                                            self.key, e
                                        );
                                    }),
                            );

                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });

                if let Some(result) = result {
                    result
                } else {
                    panic!(
                        "missing transform for import '{:?}' of package '{}'",
                        name_str, self.key
                    );
                }
            }
        };

        let new_path = DUP_SLASH_REGEX.replace_all(&new_path, |_: &Captures| "/");

        new_path.into()
    }

    fn rewrite_export(&self, old_decl: &NamedExport) -> Vec<NamedExport> {
        if old_decl.type_only || old_decl.with.is_some() {
            return vec![old_decl.clone()];
        }

        let mut out = Vec::with_capacity(old_decl.specifiers.len());

        for spec in &old_decl.specifiers {
            match spec {
                ExportSpecifier::Named(named_spec) => {
                    let name_str = named_spec.exported.as_ref().unwrap_or(&named_spec.orig);
                    let name_str = match name_str {
                        ModuleExportName::Ident(x) => x.as_ref(),
                        ModuleExportName::Str(x) => x.value.as_ref(),
                    };

                    let new_path = self.new_path(Some(name_str));
                    let specifier = if self.config.skip_default_conversion {
                        ExportSpecifier::Named(named_spec.clone())
                    } else {
                        ExportSpecifier::Named(ExportNamedSpecifier {
                            span: named_spec.span,
                            orig: ModuleExportName::Ident(Ident::new(
                                "default".into(),
                                DUMMY_SP,
                                Default::default(),
                            )),
                            exported: Some(
                                named_spec
                                    .exported
                                    .clone()
                                    .unwrap_or_else(|| named_spec.orig.clone()),
                            ),
                            is_type_only: false,
                        })
                    };
                    out.push(NamedExport {
                        specifiers: vec![specifier],
                        src: Some(Box::new(Str::from(new_path.as_ref()))),
                        span: old_decl.span,
                        type_only: false,
                        with: None,
                    });
                }
                ExportSpecifier::Namespace(ns_spec) if self.config.handle_namespace_import => {
                    let name_str = match &ns_spec.name {
                        ModuleExportName::Ident(x) => x.as_ref(),
                        ModuleExportName::Str(x) => x.value.as_ref(),
                    };
                    let new_path = self.new_path(Some(name_str));
                    let specifier = ExportSpecifier::Namespace(ns_spec.clone());
                    out.push(NamedExport {
                        specifiers: vec![specifier],
                        src: Some(Box::new(Str::from(new_path.as_ref()))),
                        span: old_decl.span,
                        type_only: false,
                        with: None,
                    });
                }
                _ => {
                    if self.config.prevent_full_import {
                        panic!(
                            "import {:?} causes the entire module to be imported",
                            old_decl
                        );
                    } else {
                        // Give up
                        return vec![old_decl.clone()];
                    }
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
                ImportSpecifier::Named(named_spec) => {
                    let name_str = named_spec
                        .imported
                        .as_ref()
                        .map(|x| match x {
                            ModuleExportName::Ident(x) => x.as_ref(),
                            ModuleExportName::Str(x) => x.value.as_ref(),
                        })
                        .unwrap_or_else(|| named_spec.local.as_ref());

                    let new_path = self.new_path(Some(name_str));
                    let specifier = if self.config.skip_default_conversion {
                        ImportSpecifier::Named(named_spec.clone())
                    } else {
                        ImportSpecifier::Default(ImportDefaultSpecifier {
                            local: named_spec.local.clone(),
                            span: named_spec.span,
                        })
                    };
                    out.push(ImportDecl {
                        specifiers: vec![specifier],
                        src: Box::new(Str::from(new_path.as_ref())),
                        span: old_decl.span,
                        type_only: false,
                        with: None,
                        phase: Default::default(),
                    });
                }
                ImportSpecifier::Namespace(ns_spec) if self.config.handle_namespace_import => {
                    let name_str = ns_spec.local.as_ref();
                    let new_path = self.new_path(Some(name_str));
                    let specifier = ImportSpecifier::Namespace(ns_spec.clone());
                    out.push(ImportDecl {
                        specifiers: vec![specifier],
                        src: Box::new(Str::from(new_path.as_ref())),
                        span: old_decl.span,
                        type_only: false,
                        with: None,
                        phase: Default::default(),
                    });
                }
                ImportSpecifier::Default(def_spec) if self.config.handle_default_import => {
                    let name_str = def_spec.local.as_ref();
                    let new_path = self.new_path(Some(name_str));
                    let specifier = ImportSpecifier::Default(def_spec.clone());
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
                    if self.config.prevent_full_import {
                        panic!(
                            "import {:?} causes the entire module to be imported",
                            old_decl
                        );
                    } else {
                        // Give up
                        return vec![old_decl.clone()];
                    }
                }
            }
        }
        out
    }
}

impl FoldImports {
    fn should_rewrite<'a>(&'a self, name: &'a str) -> Option<Rewriter<'a>> {
        for (regex, config) in &self.packages {
            let group = regex.captures(name);
            if let Some(group) = group {
                let group = group
                    .iter()
                    .map(|x| x.map(|x| x.as_str()).unwrap_or_default())
                    .collect::<Vec<&str>>();
                return Some(Rewriter {
                    renderer: &self.renderer,
                    key: name,
                    config,
                    group,
                });
            }
        }
        None
    }

    fn handle_dynamic_import(&mut self, call: &CallExpr) -> Option<Atom> {
        let first_arg = call.args.first()?;
        if first_arg.spread.is_some() {
            return None;
        }

        match &*first_arg.expr {
            Expr::Lit(Lit::Str(s)) => {
                let rewriter = self.should_rewrite(&s.value)?;
            }

            Expr::Tpl(tpl) => {
                if tpl.exprs.is_empty() {
                    if let Some(cooked) = tpl.quasis[0].cooked.as_ref() {
                        let rewriter = self.should_rewrite(cooked)?;
                    }
                }
            }
            _ => {}
        }
    }
}

impl Fold for FoldImports {
    noop_fold_type!();

    fn fold_call_expr(&mut self, mut call: CallExpr) -> CallExpr {
        call = call.fold_children_with(self);

        if call.callee.is_import() {
            if let Some(new_module) = self.handle_dynamic_import(&call) {
                call.args.first_mut().unwrap().expr = new_module.into();
            }
        }

        call
    }

    fn fold_module(&mut self, mut module: Module) -> Module {
        let mut new_items: Vec<ModuleItem> = vec![];
        for item in module.body {
            match item {
                ModuleItem::ModuleDecl(ModuleDecl::Import(decl)) => {
                    // Ignore side-effect only imports
                    if decl.specifiers.is_empty() {
                        new_items.push(ModuleItem::ModuleDecl(ModuleDecl::Import(decl)));
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
                            let rewritten = rewriter.new_path(None);
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
        module
    }
}

pub fn modularize_imports(config: Config) -> impl Pass {
    let mut folder = FoldImports {
        renderer: handlebars::Handlebars::new(),
        packages: vec![],
    };
    folder
        .renderer
        .register_helper("lowerCase", Box::new(helper_lower_case));
    folder
        .renderer
        .register_helper("upperCase", Box::new(helper_upper_case));
    folder
        .renderer
        .register_helper("camelCase", Box::new(helper_camel_case));
    folder
        .renderer
        .register_helper("kebabCase", Box::new(helper_kebab_case));
    for (mut k, v) in config.packages {
        // XXX: Should we keep this hack?
        if !k.starts_with('^') && !k.ends_with('$') {
            k = format!("^{}$", k);
        }
        folder.packages.push((
            CachedRegex::new(&k).expect("transform-imports: invalid regex"),
            v,
        ));
    }
    fold_pass(folder)
}

fn helper_lower_case(
    h: &Helper<'_>,
    _: &Handlebars<'_>,
    _: &Context,
    _: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> HelperResult {
    // get parameter from helper or throw an error
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(param.to_lowercase().as_ref())?;
    Ok(())
}

fn helper_upper_case(
    h: &Helper<'_>,
    _: &Handlebars<'_>,
    _: &Context,
    _: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> HelperResult {
    // get parameter from helper or throw an error
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(param.to_uppercase().as_ref())?;
    Ok(())
}

fn helper_camel_case(
    h: &Helper<'_>,
    _: &Handlebars<'_>,
    _: &Context,
    _: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> HelperResult {
    // get parameter from helper or throw an error
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");

    out.write(param.to_case(Case::Camel).as_ref())?;
    Ok(())
}

fn helper_kebab_case(
    h: &Helper<'_>,
    _: &Handlebars<'_>,
    _: &Context,
    _: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> HelperResult {
    // get parameter from helper or throw an error
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");

    out.write(param.to_case(Case::Kebab).as_ref())?;
    Ok(())
}

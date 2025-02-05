use rustc_hash::{FxHashMap, FxHashSet};
use swc_atoms::Atom;
use swc_common::Span;
use swc_ecma_ast::*;
use swc_ecma_visit::{noop_visit_type, Visit, VisitWith};

use crate::config::ImportItem;

#[derive(Debug, Default)]
pub(crate) struct ImportMap {
    /// Map from module name to (module path, exported symbol, span)
    imports: FxHashMap<Id, (Atom, Atom, Span)>,

    namespace_imports: FxHashMap<Id, (Atom, Span)>,

    imported_modules: FxHashSet<Atom>,
}

impl ImportMap {
    /// Returns true if `e` is an import of `orig_name` from `module`.
    pub fn is_import(&self, e: &Expr, module: &Atom, orig_name: &Atom) -> Option<Span> {
        match e {
            Expr::Ident(i) => {
                if let Some((i_src, i_sym, i_span)) = self.imports.get(&i.to_id()) {
                    if i_src == module && i_sym == orig_name {
                        Some(*i_span)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            Expr::Member(MemberExpr {
                obj: box Expr::Ident(obj),
                prop: MemberProp::Ident(prop),
                ..
            }) => {
                if let Some((obj_src, obj_span)) = self.namespace_imports.get(&obj.to_id()) {
                    if obj_src == module && prop.sym == *orig_name {
                        Some(*obj_span)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            _ => None,
        }
    }

    pub fn is_in_import_items(&self, e: &Expr, import_items: &[ImportItem]) -> Option<Span> {
        import_items
            .iter()
            .find_map(|item| self.is_import(e, &item.module, &item.name))
    }

    pub fn analyze(m: &Module) -> Self {
        let mut data = ImportMap::default();

        m.visit_with(&mut Analyzer { data: &mut data });

        data
    }
}

struct Analyzer<'a> {
    data: &'a mut ImportMap,
}

impl Visit for Analyzer<'_> {
    noop_visit_type!();

    fn visit_import_decl(&mut self, import: &ImportDecl) {
        self.data.imported_modules.insert(import.src.value.clone());

        for s in &import.specifiers {
            let (local, orig_sym) = match s {
                ImportSpecifier::Named(ImportNamedSpecifier {
                    local, imported, ..
                }) => match imported {
                    Some(imported) => (local.to_id(), orig_name(imported)),
                    _ => (local.to_id(), local.sym.clone()),
                },
                ImportSpecifier::Default(s) => (s.local.to_id(), "default".into()),
                ImportSpecifier::Namespace(s) => {
                    self.data
                        .namespace_imports
                        .insert(s.local.to_id(), (import.src.value.clone(), s.local.span));
                    continue;
                }
            };

            self.data
                .imports
                .insert(local, (import.src.value.clone(), orig_sym, import.span));
        }
    }
}

fn orig_name(n: &ModuleExportName) -> Atom {
    match n {
        ModuleExportName::Ident(v) => v.sym.clone(),
        ModuleExportName::Str(v) => v.value.clone(),
    }
}

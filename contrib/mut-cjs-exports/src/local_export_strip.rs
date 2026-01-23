use std::collections::BTreeMap;

use rustc_hash::FxHashSet;
use swc_core::{
    atoms::Atom,
    common::{util::take::Take, Span, DUMMY_SP},
    ecma::{
        ast::*,
        utils::{find_pat_ids, private_ident, ExprFactory},
        visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
    },
};

use crate::utils::{key_from_export_name, local_ident_from_export_name};

pub type Export = BTreeMap<Atom, ExportItem>;

#[derive(Debug)]
pub struct ExportItem(Span, Ident);

impl ExportItem {
    pub fn new(export_name_span: Span, local_ident: Ident) -> Self {
        Self(export_name_span, local_ident)
    }

    pub fn export_name_span(&self) -> Span {
        self.0
    }

    pub fn into_local_ident(self) -> Ident {
        self.1
    }
}

#[derive(Debug, Default)]
pub(crate) struct LocalExportStrip {
    pub(crate) has_export_assign: bool,
    pub(crate) export: Export,
    pub(crate) export_all: FxHashSet<Id>,
    pub(crate) export_decl_id: FxHashSet<Id>,
    export_default: Option<Stmt>,
}

impl VisitMut for LocalExportStrip {
    noop_visit_mut_type!(fail);

    fn visit_mut_script(&mut self, _: &mut Script) {
        // skip
    }

    fn visit_mut_module(&mut self, n: &mut Module) {
        let mut list = Vec::with_capacity(n.body.len());

        for item in n.body.drain(..) {
            match item {
                ModuleItem::Stmt(stmt) => list.push(stmt.into()),

                ModuleItem::ModuleDecl(mut module_decl) => {
                    // collect link meta
                    module_decl.visit_mut_with(self);

                    // emit stmt
                    match module_decl {
                        ModuleDecl::ExportDecl(ExportDecl { decl, .. }) => {
                            list.push(Stmt::Decl(decl).into());
                        }
                        ModuleDecl::ExportNamed(NamedExport { src: None, .. }) => continue,
                        ModuleDecl::ExportNamed(
                            item @ NamedExport {
                                src: Some(..),
                                type_only: false,
                                ..
                            },
                        ) => {
                            let decl: ModuleDecl = self.convert_export_decl(item).into();
                            list.push(decl.into());
                        }
                        ModuleDecl::ExportAll(
                            e @ ExportAll {
                                type_only: false, ..
                            },
                        ) => {
                            let decl: ModuleDecl = self.convert_export_all(e).into();
                            list.push(decl.into());
                        }
                        ModuleDecl::ExportDefaultDecl(ExportDefaultDecl {
                            decl:
                                decl @ (DefaultDecl::Class(ClassExpr {
                                    ident: Some(..), ..
                                })
                                | DefaultDecl::Fn(FnExpr {
                                    ident: Some(..), ..
                                })),
                            ..
                        }) => match decl {
                            DefaultDecl::Class(class_expr) => list.extend(
                                class_expr
                                    .as_class_decl()
                                    .map(|decl| Stmt::Decl(Decl::Class(decl)))
                                    .map(Into::into),
                            ),
                            DefaultDecl::Fn(fn_expr) => list.extend(
                                fn_expr
                                    .as_fn_decl()
                                    .map(|decl| Stmt::Decl(Decl::Fn(decl)))
                                    .map(Into::into),
                            ),
                            _ => unreachable!(),
                        },
                        ModuleDecl::ExportDefaultExpr(..) => {
                            list.extend(self.export_default.take().map(From::from))
                        }
                        ModuleDecl::TsExportAssignment(..) => {
                            self.has_export_assign = true;
                            list.push(module_decl.into());
                        }
                        _ => list.push(module_decl.into()),
                    };
                }
                #[cfg(swc_ast_unknown)]
                _ => panic!("unknown node"),
            };
        }

        n.body = list;
    }

    /// ```javascript
    /// export const foo = 1, bar = 2, { baz } = { baz: 3 };
    /// export let a = 1, [b] = [2];
    /// export function x() {}
    /// export class y {}
    /// ```
    /// ->
    /// ```javascript
    /// const foo = 1, bar = 2, { baz } = { baz: 3 };
    /// let a = 1, [b] = [2];
    /// function x() {}
    /// class y {}
    /// ```
    fn visit_mut_export_decl(&mut self, n: &mut ExportDecl) {
        match &n.decl {
            Decl::Class(ClassDecl { ident, .. }) | Decl::Fn(FnDecl { ident, .. }) => {
                self.export.insert(
                    ident.sym.clone(),
                    ExportItem::new(ident.span, ident.clone()),
                );
            }

            Decl::Var(v) => {
                let ids = find_pat_ids::<_, Ident>(&v.decls);

                self.export_decl_id.extend(ids.iter().map(Ident::to_id));

                self.export.extend(
                    find_pat_ids::<_, Ident>(&v.decls)
                        .into_iter()
                        .map(|id| (id.sym.clone(), ExportItem::new(id.span, id))),
                );
            }
            _ => {}
        };
    }

    /// ```javascript
    /// export { foo, foo as bar, foo as "baz" };
    /// export { "foo", foo as bar, "foo" as "baz" } from "mod";
    /// export * as foo from "mod";
    /// export * as "bar" from "mod";
    /// ```
    fn visit_mut_named_export(&mut self, n: &mut NamedExport) {
        if n.type_only || n.src.is_some() {
            return;
        }

        let NamedExport { specifiers, .. } = n.take();

        self.export.extend(specifiers.into_iter().map(|e| match e {
            ExportSpecifier::Namespace(..) => {
                unreachable!("`export *` without src is invalid")
            }
            ExportSpecifier::Default(..) => {
                unreachable!("`export foo` without src is invalid")
            }
            ExportSpecifier::Named(ExportNamedSpecifier { orig, exported, .. }) => {
                let orig = match orig {
                    ModuleExportName::Ident(id) => id,
                    ModuleExportName::Str(_) => {
                        unreachable!(r#"`export {{ "foo" }}` without src is invalid"#)
                    }
                    #[cfg(swc_ast_unknown)]
                    _ => panic!("unknown node"),
                };

                if let Some(exported) = exported {
                    let (export_name, export_name_span) = match exported {
                        ModuleExportName::Ident(Ident { span, sym, .. }) => (sym, span),
                        ModuleExportName::Str(Str { span, value, .. }) => (
                            match value.as_atom() {
                                Some(s) => s.clone(),
                                None => panic!("non-utf8 export name: {value:?}"),
                            },
                            span,
                        ),
                        #[cfg(swc_ast_unknown)]
                        _ => panic!("unknown node"),
                    };

                    (export_name, ExportItem::new(export_name_span, orig))
                } else {
                    (orig.sym.clone(), ExportItem::new(orig.span, orig))
                }
            }
            #[cfg(swc_ast_unknown)]
            _ => panic!("unknown node"),
        }))
    }

    /// ```javascript
    /// export default class foo {};
    /// export default class {};
    /// export default function bar () {};
    /// export default function () {};
    /// ```
    /// ->
    /// ```javascript
    /// class foo {};
    /// class _default {};
    /// function bar () {};
    /// function _default () {};
    /// ```
    fn visit_mut_export_default_decl(&mut self, n: &mut ExportDefaultDecl) {
        match &mut n.decl {
            DefaultDecl::Class(class_expr) => {
                if let Some(ident) = class_expr.ident.clone() {
                    self.export
                        .insert("default".into(), ExportItem::new(n.span, ident));
                }
            }
            DefaultDecl::Fn(fn_expr) => {
                if let Some(ident) = fn_expr.ident.clone() {
                    self.export
                        .insert("default".into(), ExportItem::new(n.span, ident));
                }
            }
            DefaultDecl::TsInterfaceDecl(_) => {}
            #[cfg(swc_ast_unknown)]
            _ => panic!("unknown node"),
        }
    }

    /// ```javascript
    /// export default foo;
    /// export default 1
    /// ```
    /// ->
    /// ```javascript
    /// var _default = foo;
    /// var _default = 1;
    /// ```
    fn visit_mut_export_default_expr(&mut self, n: &mut ExportDefaultExpr) {
        let ident = private_ident!(n.span, "_default");

        self.export
            .insert("default".into(), ExportItem::new(n.span, ident.clone()));

        self.export_default = Some(Stmt::Decl(
            n.expr
                .take()
                .into_var_decl(VarDeclKind::Const, ident.into())
                .into(),
        ));
    }
}

impl LocalExportStrip {
    fn convert_export_decl(&mut self, n: NamedExport) -> ImportDecl {
        let NamedExport {
            span,
            specifiers,
            src,
            type_only,
            with,
        } = n;

        let src = src.unwrap();

        let specifiers = specifiers
            .into_iter()
            .flat_map(|s| self.convert_export_specifier(s))
            .collect();

        ImportDecl {
            span,
            specifiers,
            src,
            type_only,
            with,
            phase: Default::default(),
        }
    }

    fn convert_export_specifier(&mut self, s: ExportSpecifier) -> Option<ImportSpecifier> {
        match s {
            ExportSpecifier::Namespace(ExportNamespaceSpecifier { span, name }) => {
                let (export_name, export_span) = key_from_export_name(&name);
                let local = local_ident_from_export_name(name);
                self.export
                    .insert(export_name, ExportItem::new(export_span, local.clone()));

                Some(ImportSpecifier::Namespace(ImportStarAsSpecifier {
                    span,
                    local,
                }))
            }
            ExportSpecifier::Default(ExportDefaultSpecifier { exported }) => {
                let (export_name, export_span) = (exported.sym.clone(), exported.span);
                let local = exported.into_private();
                self.export
                    .insert(export_name, ExportItem::new(export_span, local.clone()));

                Some(ImportSpecifier::Default(ImportDefaultSpecifier {
                    local,
                    span: DUMMY_SP,
                }))
            }
            ExportSpecifier::Named(ExportNamedSpecifier {
                span,
                orig,
                exported,
                is_type_only: false,
            }) => {
                // export { "x-1" as "y-1" } from "foo"
                // ->
                // import { "x-1" as x1 } from "foo"
                // export { x1 as "y-1" }
                let name = exported.as_ref().unwrap_or(&orig);

                let (export_name, export_span) = key_from_export_name(name);
                let local = local_ident_from_export_name(orig.clone());
                self.export
                    .insert(export_name, ExportItem::new(export_span, local.clone()));

                Some(ImportSpecifier::Named(ImportNamedSpecifier {
                    span,
                    local,
                    imported: Some(orig),
                    is_type_only: false,
                }))
            }
            _ => None,
        }
    }

    fn convert_export_all(&mut self, e: ExportAll) -> ImportDecl {
        let ExportAll {
            span, src, with, ..
        } = e;

        let mod_name = private_ident!("mod");

        self.export_all.insert(mod_name.to_id());

        let star = ImportStarAsSpecifier {
            span,
            local: mod_name.clone(),
        };

        ImportDecl {
            span,
            specifiers: vec![star.into()],
            src,
            type_only: false,
            with,
            phase: Default::default(),
        }
    }
}

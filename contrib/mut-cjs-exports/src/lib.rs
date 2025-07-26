mod local_export_strip;
mod utils;

use local_export_strip::LocalExportStrip;
use rustc_hash::FxHashSet;
use swc_core::{
    common::{util::take::Take, Mark, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::*,
        utils::{
            for_each_binding_ident, member_expr, private_ident, quote_ident, quote_str,
            ExprFactory, IntoIndirectCall,
        },
        visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
use utils::{emit_export_stmts, object_define_property};

#[derive(Debug)]
pub struct TransformVisitor {
    unresolved_mark: Mark,

    export_decl_id: FxHashSet<Id>,
}

impl VisitMut for TransformVisitor {
    noop_visit_mut_type!(fail);

    fn visit_mut_script(&mut self, _: &mut Script) {
        // skip
    }

    fn visit_mut_module(&mut self, n: &mut Module) {
        let mut strip = LocalExportStrip::default();
        n.visit_mut_with(&mut strip);

        let LocalExportStrip {
            has_export_assign,
            export,
            export_all,
            export_decl_id,
            ..
        } = strip;

        self.export_decl_id = export_decl_id;

        let mut stmts: Vec<ModuleItem> = Vec::with_capacity(n.body.len() + 1);

        if !has_export_assign && !export.is_empty() {
            // keep module env
            stmts.push(ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(
                NamedExport::dummy(),
            )));

            let exports = self.exports();

            stmts.extend(
                emit_export_stmts(exports, export)
                    .into_iter()
                    .map(Into::into),
            );

            if !self.export_decl_id.is_empty() {
                n.visit_mut_children_with(self);
            }
        }

        stmts.extend(export_all.into_iter().map(|id| self.export_all(id)));

        stmts.extend(n.body.take());

        n.body = stmts;
    }

    fn visit_mut_function(&mut self, node: &mut Function) {
        let export_decl_id = self.export_decl_id.clone();

        for_each_binding_ident(&node.params, |ident| {
            self.export_decl_id.remove(&ident.id.to_id());
        });

        node.visit_mut_children_with(self);
        self.export_decl_id = export_decl_id;
    }

    fn visit_mut_prop(&mut self, n: &mut Prop) {
        match n {
            Prop::Shorthand(ref_ident) => {
                if self.export_decl_id.contains(&ref_ident.to_id()) {
                    *n = KeyValueProp {
                        key: ref_ident.clone().into(),
                        value: Box::new(self.exports().make_member(ref_ident.take().into()).into()),
                    }
                    .into()
                }
            }
            _ => n.visit_mut_children_with(self),
        }
    }

    fn visit_mut_expr(&mut self, n: &mut Expr) {
        match n {
            Expr::Ident(ref_ident) => {
                if self.export_decl_id.contains(&ref_ident.to_id()) {
                    *n = self.exports().make_member(ref_ident.take().into()).into();
                }
            }

            _ => n.visit_mut_children_with(self),
        };
    }

    fn visit_mut_tagged_tpl(&mut self, n: &mut TaggedTpl) {
        let is_indirect = n
            .tag
            .as_ident()
            .map(|ident| self.export_decl_id.contains(&ident.to_id()))
            .unwrap_or_default();

        n.visit_mut_children_with(self);

        if is_indirect {
            *n = n.take().into_indirect()
        }
    }

    fn visit_mut_callee(&mut self, n: &mut Callee) {
        match n {
            Callee::Expr(e) if e.is_ident() => {
                let is_indirect_callee = e
                    .as_ident()
                    .map(|ident| self.export_decl_id.contains(&ident.to_id()))
                    .unwrap_or_default();

                e.visit_mut_with(self);

                if is_indirect_callee {
                    *n = n.take().into_indirect()
                }
            }

            _ => n.visit_mut_children_with(self),
        }
    }

    fn visit_mut_jsx_element_name(&mut self, n: &mut JSXElementName) {
        match n {
            JSXElementName::Ident(ref_ident) => {
                if self.export_decl_id.contains(&ref_ident.to_id()) {
                    *n = JSXElementName::JSXMemberExpr(JSXMemberExpr {
                        span: DUMMY_SP,
                        obj: self.exports().into(),
                        prop: ref_ident.clone().into(),
                    });
                }
            }
            _ => n.visit_mut_children_with(self),
        };
    }
}

impl TransformVisitor {
    pub fn new(unresolved_mark: Mark) -> Self {
        Self {
            unresolved_mark,
            export_decl_id: Default::default(),
        }
    }

    fn exports(&self) -> Ident {
        quote_ident!(
            SyntaxContext::empty().apply_mark(self.unresolved_mark),
            "exports"
        )
    }

    /// ```JavaScript
    /// Object.keys(_mod).forEach(function (key) {
    ///     if (key === "default" || key === "__esModule") return;
    ///     if (Object.prototype.hasOwnProperty.call(exports, key)) return;
    ///     Object.defineProperty(exports, key, {
    ///         enumerable: true,
    ///         get: function () {
    ///             return mod[key];
    ///         },
    ///         configurable: true
    ///     });
    /// ```
    fn export_all(&self, id: Id) -> ModuleItem {
        let mod_name = Ident::from(id);
        let key = private_ident!("key");

        member_expr!(Default::default(), DUMMY_SP, Object.keys)
            .as_call(DUMMY_SP, vec![mod_name.clone().as_arg()])
            .make_member(quote_ident!("forEach"))
            .as_call(
                DUMMY_SP,
                vec![Function {
                    params: vec![key.clone().into()],
                    span: DUMMY_SP,
                    body: Some(BlockStmt {
                        stmts: vec![
                            // if (key === "default" || key === "__esModule") return;
                            IfStmt {
                                span: DUMMY_SP,
                                test: BinExpr {
                                    span: DUMMY_SP,
                                    op: op!("||"),
                                    left: BinExpr {
                                        span: DUMMY_SP,
                                        op: op!("==="),
                                        left: key.clone().into(),
                                        right: quote_str!("default").into(),
                                    }
                                    .into(),
                                    right: BinExpr {
                                        span: DUMMY_SP,
                                        op: op!("==="),
                                        left: key.clone().into(),
                                        right: quote_str!("__esModule").into(),
                                    }
                                    .into(),
                                }
                                .into(),
                                cons: Box::new(
                                    ReturnStmt {
                                        span: DUMMY_SP,
                                        arg: None,
                                    }
                                    .into(),
                                ),
                                alt: None,
                            }
                            .into(),
                            // if (Object.prototype.hasOwnProperty.call(exports, key)) return;
                            IfStmt {
                                span: DUMMY_SP,
                                test: Box::new(
                                    member_expr!(
                                        Default::default(),
                                        DUMMY_SP,
                                        Object.prototype.hasOwnProperty.call
                                    )
                                    .as_call(
                                        DUMMY_SP,
                                        vec![self.exports().as_arg(), key.clone().as_arg()],
                                    ),
                                ),
                                cons: Box::new(
                                    ReturnStmt {
                                        span: DUMMY_SP,
                                        arg: None,
                                    }
                                    .into(),
                                ),
                                alt: None,
                            }
                            .into(),
                            // Object.defineProperty(exports, key, {
                            //     enumerable: true,
                            //     get: function () {
                            //       return mod[key];
                            //     },
                            //     configurable: true
                            //   });
                            object_define_property(
                                self.exports().as_arg(),
                                key.clone().as_arg(),
                                ObjectLit {
                                    span: DUMMY_SP,
                                    props: vec![
                                        PropOrSpread::Prop(Box::new(
                                            KeyValueProp {
                                                key: quote_ident!("enumerable").into(),
                                                value: true.into(),
                                            }
                                            .into(),
                                        )),
                                        PropOrSpread::Prop(Box::new(
                                            KeyValueProp {
                                                key: quote_ident!("get").into(),
                                                value: mod_name
                                                    .clone()
                                                    .computed_member(key.clone())
                                                    .into_lazy_fn(vec![])
                                                    .into(),
                                            }
                                            .into(),
                                        )),
                                        PropOrSpread::Prop(Box::new(
                                            KeyValueProp {
                                                key: quote_ident!("configurable").into(),
                                                value: true.into(),
                                            }
                                            .into(),
                                        )),
                                    ],
                                }
                                .as_arg(),
                            )
                            .into_stmt(),
                        ],
                        ..Default::default()
                    }),
                    ..Default::default()
                }
                .as_arg()],
            )
            .into_stmt()
            .into()
    }
}

#[plugin_transform]
pub fn process_transform(
    mut program: Program,
    metadata: TransformPluginProgramMetadata,
) -> Program {
    program.visit_mut_with(&mut TransformVisitor::new(metadata.unresolved_mark));
    program
}

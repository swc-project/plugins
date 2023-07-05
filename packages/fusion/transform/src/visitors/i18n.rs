use std::{cell::RefCell, rc::Rc};

use swc_core::{
    common::{FileName, DUMMY_SP},
    ecma::{
        ast::*,
        utils::prepend_stmt,
        visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
    },
};
use tracing::debug;

use crate::i18n::State;

pub fn i18n_report_ids(file_name: FileName, state: Rc<RefCell<State>>) -> impl Fold + VisitMut {
    as_folder(DisplayNameAndId { file_name, state })
}

#[derive(Debug)]
struct DisplayNameAndId {
    file_name: FileName,
    state: Rc<RefCell<State>>,
}

impl VisitMut for DisplayNameAndId {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        n.visit_mut_children_with(self);
        let analyzer_state = self.state.borrow();
        debug!("state from the analyzer {:?}", analyzer_state);
        let mut translation_ids: Vec<_> =
            analyzer_state.get_translation_ids().into_iter().collect();
        translation_ids.sort();
        let translation_ids_tpl = analyzer_state.get_translation_ids_tpl();
        if !translation_ids.is_empty() || !translation_ids_tpl.is_empty() {
            prepend_stmt(
                &mut n.body,
                ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                    span: DUMMY_SP,
                    specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
                        span: DUMMY_SP,
                        local: Ident {
                            span: DUMMY_SP,
                            sym: "_fusionPluginI18nChunkTranslationMap".into(),
                            optional: false,
                        },
                    })],
                    src: Box::new(Str {
                        span: DUMMY_SP,
                        value: "virtual:fusion-vite-i18n-map".into(),
                        raw: None,
                    }),
                    type_only: Default::default(),
                    asserts: Default::default(),
                })),
            );

            n.body.push(ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: Box::new(Expr::Call(CallExpr {
                    span: DUMMY_SP,
                    callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                        span: DUMMY_SP,
                        obj: Box::new(Expr::Ident(Ident::new(
                            "_fusionPluginI18nChunkTranslationMap".into(),
                            DUMMY_SP,
                        ))),
                        prop: MemberProp::Ident(Ident::new("add".into(), DUMMY_SP)),
                    }))),
                    args: vec![
                        ExprOrSpread {
                            spread: None,
                            expr: Box::new(Expr::Lit(Lit::Str(Str {
                                span: DUMMY_SP,
                                raw: None,
                                value: self.file_name.to_string().into(),
                            }))),
                        },
                        ExprOrSpread {
                            spread: None,
                            expr: Box::new(Expr::Array(ArrayLit {
                                span: DUMMY_SP,
                                elems: vec![Some(ExprOrSpread {
                                    spread: None,
                                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                                        span: DUMMY_SP,
                                        raw: None,
                                        value: "vite-i18n-chunk".into(),
                                    }))),
                                })],
                            })),
                        },
                        ExprOrSpread {
                            spread: None,
                            expr: Box::new(Expr::Array(ArrayLit {
                                span: DUMMY_SP,
                                elems: translation_ids
                                    .into_iter()
                                    .map(|s| {
                                        Some(ExprOrSpread {
                                            spread: None,
                                            expr: Box::new(Expr::Lit(Lit::Str(Str {
                                                span: DUMMY_SP,
                                                value: s.to_string().into(),
                                                raw: None,
                                            }))),
                                        })
                                    })
                                    .chain(translation_ids_tpl.into_iter().map(|inner_set| {
                                        Some(ExprOrSpread {
                                            spread: None,
                                            expr: Box::new(Expr::Array(ArrayLit {
                                                span: DUMMY_SP,
                                                elems: inner_set
                                                    .into_iter()
                                                    .rev()
                                                    .map(|s| {
                                                        Some(ExprOrSpread {
                                                            spread: None,
                                                            expr: Box::new(Expr::Lit(Lit::Str(
                                                                Str {
                                                                    span: DUMMY_SP,
                                                                    value: s.to_string().into(),
                                                                    raw: None,
                                                                },
                                                            ))),
                                                        })
                                                    })
                                                    .collect(),
                                            })),
                                        })
                                    }))
                                    .collect(),
                            })),
                        },
                    ],
                    type_args: None,
                })),
            })));
        }
    }
}

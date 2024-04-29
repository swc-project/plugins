//! Port of https://github.com/styled-components/babel-plugin-styled-components/blob/4e2eb388d9c90f2921c306c760657d059d01a518/src/visitors/templateLiterals/transpile.js

use std::{cell::RefCell, iter, rc::Rc};

use swc_common::{util::take::Take, DUMMY_SP};
use swc_ecma_ast::*;
use swc_ecma_visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith};

use crate::utils::State;

pub fn template_literals(state: Rc<RefCell<State>>) -> impl Fold + VisitMut {
    as_folder(TemplateLiterals { state })
}

#[derive(Debug)]
struct TemplateLiterals {
    state: Rc<RefCell<State>>,
}

impl VisitMut for TemplateLiterals {
    noop_visit_mut_type!();

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        let Expr::TaggedTpl(tagged) = expr else {
            return;
        };
        if !self.state.borrow().is_styled(&tagged.tag)
            && !self.state.borrow().is_helper(&tagged.tag)
        {
            return;
        }

        expr.map_with_mut(|expr| {
            let tagged = expr.expect_tagged_tpl();

            let quasis = tagged
                .tpl
                .quasis
                .into_iter()
                .map(|q| {
                    Expr::Tpl(Tpl {
                        span: q.span,
                        exprs: vec![],
                        quasis: vec![q],
                    })
                })
                .map(ExprOrSpread::from)
                .map(Some);
            let exprs = tagged.tpl.exprs.into_iter().map(ExprOrSpread::from);
            let args = iter::once(
                Expr::Array(ArrayLit {
                    span: DUMMY_SP,
                    elems: quasis.collect(),
                })
                .into(),
            )
            .chain(exprs)
            .collect();

            Expr::Call(CallExpr {
                span: tagged.span,
                callee: tagged.tag.into(),
                args,
                type_args: tagged.type_params,
            })
        });
    }
}

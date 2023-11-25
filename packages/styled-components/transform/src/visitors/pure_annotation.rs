//! Port of https://github.com/styled-components/babel-plugin-styled-components/blob/4e2eb388d9c90f2921c306c760657d059d01a518/src/visitors/pure.js

use std::{cell::RefCell, rc::Rc};

use swc_common::{comments::Comments, Spanned, DUMMY_SP};
use swc_ecma_ast::*;
use swc_ecma_visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith};

use crate::utils::State;

pub fn pure_annotation<C>(comments: C, state: Rc<RefCell<State>>) -> impl Fold + VisitMut
where
    C: Comments,
{
    as_folder(PureAnnotation { comments, state })
}

#[derive(Debug)]
struct PureAnnotation<C>
where
    C: Comments,
{
    comments: C,
    state: Rc<RefCell<State>>,
}

impl<C> VisitMut for PureAnnotation<C>
where
    C: Comments,
{
    noop_visit_mut_type!();

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        let is_styled_or_pure_helper = match expr {
            Expr::Call(CallExpr {
                callee: Callee::Expr(callee),
                ..
            }) => {
                self.state.borrow().is_styled(callee) || self.state.borrow().is_pure_helper(callee)
            }
            Expr::TaggedTpl(tagged) => {
                self.state.borrow().is_styled(&tagged.tag)
                    || self.state.borrow().is_pure_helper(&tagged.tag)
            }
            _ => false,
        };
        if !is_styled_or_pure_helper {
            return;
        }

        let span = expr.span();
        if span == DUMMY_SP {
            return;
        }
        if !self.comments.has_flag(span.lo, "PURE") {
            self.comments.add_pure_comment(span.lo);
        }
    }
}

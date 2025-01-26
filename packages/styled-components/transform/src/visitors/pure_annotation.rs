//! Port of https://github.com/styled-components/babel-plugin-styled-components/blob/4e2eb388d9c90f2921c306c760657d059d01a518/src/visitors/pure.js

use swc_common::{comments::Comments, Span};
use swc_ecma_ast::*;
use swc_ecma_visit::{noop_visit_mut_type, visit_mut_pass, VisitMut, VisitMutWith};

use crate::utils::State;

pub fn pure_annotation<C>(comments: C, state: &State) -> impl '_ + Pass
where
    C: Comments,
{
    visit_mut_pass(PureAnnotation { comments, state })
}

#[derive(Debug)]
struct PureAnnotation<'a, C>
where
    C: Comments,
{
    comments: C,
    state: &'a State,
}

impl<'a, C> VisitMut for PureAnnotation<'a, C>
where
    C: Comments,
{
    noop_visit_mut_type!();

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        let (callee_or_tag, span) = match expr {
            Expr::Call(CallExpr {
                span,
                callee: Callee::Expr(callee),
                ..
            }) => (callee, span),
            Expr::TaggedTpl(TaggedTpl { span, tag, .. }) => (tag, span),
            _ => return,
        };
        if !self.state.is_styled(callee_or_tag) && !self.state.is_pure_helper(callee_or_tag) {
            return;
        }

        if span.is_dummy_ignoring_cmt() {
            *span = Span::dummy_with_cmt();
        }
        if !self.comments.has_flag(span.lo, "PURE") {
            self.comments.add_pure_comment(span.lo);
        }
    }
}

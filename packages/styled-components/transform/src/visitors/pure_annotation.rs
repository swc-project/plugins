//! Port of https://github.com/styled-components/babel-plugin-styled-components/blob/4e2eb388d9c90f2921c306c760657d059d01a518/src/visitors/pure.js

use swc_common::{comments::Comments, BytePos, Span};
use swc_ecma_ast::*;
use swc_ecma_visit::{noop_visit_mut_type, visit_mut_pass, VisitMut, VisitMutWith};

use crate::utils::State;

pub fn pure_annotation<'a, C>(comments: C, state: &'a State) -> impl 'a + Pass
where
    C: 'a + Comments,
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

impl<C> VisitMut for PureAnnotation<'_, C>
where
    C: Comments,
{
    noop_visit_mut_type!(fail);

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

        if span.is_dummy_ignoring_cmt() || starts_with_call_at(callee_or_tag, span.lo) {
            *span = Span::dummy_with_cmt();
        }
        if !self.comments.has_flag(span.lo, "PURE") {
            self.comments.add_pure_comment(span.lo);
        }
    }
}

/// Returns `true` if a call, `new` expression or tagged template nested in
/// `expr` starts at `lo`.
///
/// The `PURE` annotation is stored by position, so annotating an expression
/// which shares its start position with such a nested expression is ambiguous.
/// The fixer resolves the ambiguity in favor of the nested expression by
/// parenthesizing it, and the annotation then applies to the receiver only
/// instead of the whole chained call. We want the whole
/// `styled(Inner).withConfig({})([...])` call to be annotated, not just
/// `styled(Inner)`.
fn starts_with_call_at(expr: &Expr, lo: BytePos) -> bool {
    let mut cur = expr;

    loop {
        cur = match cur {
            Expr::Call(CallExpr {
                span,
                callee: Callee::Expr(callee),
                ..
            }) => {
                if span.lo == lo {
                    return true;
                }
                callee
            }

            Expr::New(NewExpr { span, callee, .. }) => {
                if span.lo == lo {
                    return true;
                }
                callee
            }

            Expr::TaggedTpl(TaggedTpl { span, tag, .. }) => {
                if span.lo == lo {
                    return true;
                }
                tag
            }

            Expr::Member(MemberExpr { obj, .. }) => obj,
            Expr::Paren(ParenExpr { expr, .. }) => expr,

            _ => return false,
        };
    }
}

//! Adds a parent selector wrapper to styled-components template literals.

use swc_ecma_ast::*;
use swc_ecma_visit::{noop_visit_mut_type, visit_mut_pass, VisitMut, VisitMutWith};

use crate::utils::State;

pub fn css_namespace(state: &State, selector: String) -> impl '_ + Pass {
    visit_mut_pass(CssNamespace { state, selector })
}

#[derive(Debug)]
struct CssNamespace<'a> {
    state: &'a State,
    selector: String,
}

impl VisitMut for CssNamespace<'_> {
    noop_visit_mut_type!(fail);

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        let Expr::TaggedTpl(tagged) = expr else {
            return;
        };

        if !self.state.is_styled(&tagged.tag) {
            return;
        }

        wrap_template(&mut tagged.tpl, &self.selector);
    }
}

fn wrap_template(tpl: &mut Tpl, selector: &str) {
    if let Some(first) = tpl.quasis.first_mut() {
        let raw = first.raw.to_string();
        first.raw = format!("{selector} {{{raw}").into();
        first.cooked = None;
    }

    if let Some(last) = tpl.quasis.last_mut() {
        let raw = last.raw.to_string();
        last.raw = format!("{raw}}}").into();
        last.cooked = None;
    }
}

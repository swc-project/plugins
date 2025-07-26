//! Port of https://github.com/styled-components/babel-plugin-styled-components/blob/4e2eb388d9c90f2921c306c760657d059d01a518/src/visitors/minify.js

use swc_common::DUMMY_SP;
use swc_ecma_ast::*;
use swc_ecma_visit::{noop_visit_mut_type, visit_mut_pass, VisitMut, VisitMutWith};

use super::css::{minify_raw_values, MinifyResult};
use crate::utils::State;

pub fn minify(state: &State) -> impl '_ + Pass {
    visit_mut_pass(Minify { state })
}

#[derive(Debug)]
struct Minify<'a> {
    state: &'a State,
}

impl VisitMut for Minify<'_> {
    noop_visit_mut_type!(fail);

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        let Expr::TaggedTpl(tagged) = expr else {
            return;
        };
        if !self.state.is_styled(&tagged.tag) && !self.state.is_helper(&tagged.tag) {
            return;
        }

        let MinifyResult {
            values: raw_values_minified,
            retained_expression_indices,
        } = minify_raw_values(tagged.tpl.quasis.iter().map(|q| q.raw.clone()));

        tagged.tpl.quasis = raw_values_minified
            .into_iter()
            .map(|raw| TplElement {
                span: DUMMY_SP,
                tail: false,
                // Omitting `cooked` field since swc_ecma_codegen doesn't use it. If this breaks
                // other plugins, we may need to set some value.
                // https://rustdoc.swc.rs/swc_ecma_ast/struct.TplElement.html#structfield.cooked
                cooked: None,
                raw,
            })
            .collect();
        if let Some(q) = tagged.tpl.quasis.last_mut() {
            q.tail = true;
        }

        // Remove expressions that were removed by minification.
        // NOTE: Here we assume that the expressions don't have side effects, as
        // babel-plugin-styled-components does.
        {
            let mut idx: usize = 0;
            tagged.tpl.exprs.retain(|_| {
                idx += 1;
                retained_expression_indices.contains(&(idx - 1))
            });
        }
    }
}

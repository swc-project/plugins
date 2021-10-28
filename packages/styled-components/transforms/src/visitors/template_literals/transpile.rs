//! Port of https://github.com/styled-components/babel-plugin-styled-components/blob/a20c3033508677695953e7a434de4746168eeb4e/src/visitors/transpileCssProp.js

use swc_ecmascript::{
    ast::*,
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
};

pub fn transpile_css_prop() -> impl Fold + VisitMut {
    as_folder(TranspileCssProp)
}

struct TranspileCssProp;

impl VisitMut for TranspileCssProp {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        // TODO: Skip if there are no css prop usage
        n.visit_mut_children_with(self);
    }

    fn visit_mut_script(&mut self, n: &mut Script) {
        // TODO: Skip if there are no css prop usage
        n.visit_mut_children_with(self);
    }

    fn visit_mut_jsx_attr(&mut self, n: &mut JSXAttr) {
        n.visit_mut_children_with(self);

        if n.name != "css" {
            return;
        }

        n.visit_mut_with(&mut transpile_css_prop());
    }
}

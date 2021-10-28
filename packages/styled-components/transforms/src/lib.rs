use swc_ecmascript::{
    ast::JSXAttr,
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
};
use visitors::template_literals::transpile::transpile_css_prop;

mod css;
mod utils;
mod visitors;

pub fn styled_components() -> impl Fold + VisitMut {
    as_folder(StyledComponents)
}

struct StyledComponents;

impl VisitMut for StyledComponents {
    noop_visit_mut_type!();

    fn visit_mut_jsx_attr(&mut self, n: &mut JSXAttr) {
        n.visit_mut_children_with(self);

        n.visit_mut_with(&mut transpile_css_prop());
    }
}

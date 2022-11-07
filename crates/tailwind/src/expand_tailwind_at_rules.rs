use swc_core::css::{
    ast::Stylesheet,
    visit::{VisitMut, VisitMutWith},
};

pub(crate) fn expand_tailwind_at_rules(ss: &mut Stylesheet) {
    ss.visit_mut_with(&mut TailwindExpander::default());
}

#[derive(Debug, Default)]
struct TailwindExpander {}

impl VisitMut for TailwindExpander {}

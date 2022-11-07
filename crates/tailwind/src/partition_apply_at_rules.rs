use swc_core::css::{
    ast::Stylesheet,
    visit::{VisitMut, VisitMutWith},
};

pub fn partition_apply_at_rules(ss: &mut Stylesheet) {
    ss.visit_mut_with(&mut Visitor::default());
}

#[derive(Debug, Default)]
struct Visitor {}

impl VisitMut for Visitor {}

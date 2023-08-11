use swc_core::ecma::visit::{noop_visit_mut_type, VisitMut};

pub fn constify() -> impl VisitMut {
    Constify::default()
}

#[derive(Default)]
struct Constify {}

impl VisitMut for Constify {
    noop_visit_mut_type!();
}

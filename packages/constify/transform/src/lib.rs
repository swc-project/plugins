use swc_core::ecma::visit::{noop_visit_mut_type, VisitMut};

pub fn transform() -> impl VisitMut {
    Transform::default()
}

#[derive(Default)]
struct Transform {}

impl VisitMut for Transform {
    noop_visit_mut_type!();
}

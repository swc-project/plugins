#![allow(unused)]

use swc_core::{
    common::errors::HANDLER,
    css::{
        ast::{AtRule, Stylesheet},
        visit::{Visit, VisitWith},
    },
};

/// Emits error for `@tailwind` directives in nested rules.
pub(crate) fn detect_nesting(ss: &Stylesheet) {
    // ss.visit_with(&mut Detector {
    //     in_nested_rule: false,
    //     found: false,
    // });
}

struct Detector {
    in_nested_rule: bool,
    found: bool,
}

// TODO(kdy1): Report error for nested css
impl Visit for Detector {}

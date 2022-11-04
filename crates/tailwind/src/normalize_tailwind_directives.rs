use swc_core::{
    common::collections::AHashSet,
    css::ast::{AtRule, Stylesheet},
    ecma::atoms::JsWord,
};

pub(crate) struct Directives {
    pub tailwindDirectives: AHashSet<JsWord>,

    pub applyDirectives: AHashSet<AtRule>,
}

pub(crate) fn normalize_tailwind_directives(ss: &Stylesheet) -> Directives {}

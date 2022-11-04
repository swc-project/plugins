use swc_core::css::ast::Stylesheet;

pub use crate::config::*;
use crate::{
    detect_nesting::detect_nesting, normalize_tailwind_directives::normalize_tailwind_directives,
};

mod config;
mod detect_nesting;
mod normalize_tailwind_directives;

/// Main entrypoint.
///
/// Note: We don't find config file here. It should be done by the caller.
pub struct Compiler {
    config: Config,
}

impl Compiler {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn process(&self, ss: &mut Stylesheet) {
        normalize_tailwind_directives(ss);

        detect_nesting(ss);
    }
}

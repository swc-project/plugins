#![feature(box_patterns)]
#![feature(box_syntax)]

use swc_core::css::ast::Stylesheet;

pub use crate::config::*;
use crate::{
    detect_nesting::detect_nesting, expand_tailwind_at_rules::expand_tailwind_at_rules,
    normalize_tailwind_directives::normalize_tailwind_directives,
    partition_apply_at_rules::partition_apply_at_rules,
};

mod config;
mod detect_nesting;
mod expand_tailwind_at_rules;
mod normalize_tailwind_directives;
mod partition_apply_at_rules;

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
        let directives = normalize_tailwind_directives(ss);

        detect_nesting(ss);

        // Partition apply rules that are found in the css itself.
        partition_apply_at_rules(ss);

        expand_tailwind_at_rules(ss);
    }
}

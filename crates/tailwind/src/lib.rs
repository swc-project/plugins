#![feature(box_patterns)]
#![feature(box_syntax)]

use swc_core::css::ast::Stylesheet;

pub use crate::config::*;
use crate::{
    context::Context, detect_nesting::detect_nesting,
    evaluate_tailwind_functions::evaluate_tailwind_functions,
    expand_apply_at_rules::expand_apply_at_rules,
    expand_tailwind_at_rules::expand_tailwind_at_rules,
    normalize_tailwind_directives::normalize_tailwind_directives,
    partition_apply_at_rules::partition_apply_at_rules,
};

mod base;
mod config;
mod context;
mod detect_nesting;
mod evaluate_tailwind_functions;
mod expand_apply_at_rules;
mod expand_tailwind_at_rules;
mod generate_rules;
mod normalize_tailwind_directives;
mod partition_apply_at_rules;
mod util;

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

        let mut context = Context {
            tailwind_config: &self.config,
        };

        expand_tailwind_at_rules(&mut context, ss);

        // Partition apply rules that are generated by addComponents, addUtilities and
        // so on.
        partition_apply_at_rules(ss);
        expand_apply_at_rules(&mut context, ss);
        evaluate_tailwind_functions(&mut context, ss);
        substitute_screen_at_rules(context, ss);
        resolve_defaults_at_rules(context, ss);
        collapse_adjacent_rules(context, ss);
        collapse_duplicate_declarations(context, ss);
    }
}

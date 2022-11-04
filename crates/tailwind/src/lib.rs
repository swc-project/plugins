use swc_core::css::visit::VisitMut;

mod detect_nesting;

pub struct Config {}

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
}

use swc_core::css::visit::VisitMut;

pub struct Config {}

/// Main entrypoint.
///
/// Note: We don't find config file here. It should be done by the caller.
pub fn tailwind(c: Config) -> impl VisitMut {}

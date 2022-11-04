use serde::Deserialize;
use swc_core::css::ast::Stylesheet;

use crate::{
    detect_nesting::detect_nesting, normalize_tailwind_directives::normalize_tailwind_directives,
};

mod detect_nesting;
mod normalize_tailwind_directives;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub content: Vec<Content>,
    pub theme: ThemeConfig,

    pub core_plugins: CorePluginsConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ThemeConfig {}

impl Default for ThemeConfig {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CorePluginsConfig {
    #[serde(default = "true_by_default")]
    pub preflight: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Content {
    Raw { raw: String },
}

const fn true_by_default() -> bool {
    true
}

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

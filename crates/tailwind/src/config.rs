use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub content: Vec<Content>,

    #[serde(default)]
    pub theme: ThemeConfig,

    pub core_plugins: CorePluginsConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ThemeConfig {}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {}
    }
}

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

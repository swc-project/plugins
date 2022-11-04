use serde::Deserialize;
use swc_core::common::collections::AHashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub content: ContentConfig,

    #[serde(default)]
    pub theme: ThemeConfig,

    #[serde(default)]
    pub core_plugins: CorePluginsConfig,

    #[serde(default = "default_prefix")]
    pub prefix: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum ContentConfig {
    Files(Vec<Content>),
    Raw { files: Vec<Content> },
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ThemeConfig {
    #[serde(default)]
    pub extend: Option<Box<ThemeConfig>>,

    #[serde(default)]
    pub screens: AHashMap<String, String>,

    #[serde(default)]
    pub keyframes: AHashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum CorePluginsConfig {
    Map {
        #[serde(default = "true_by_default")]
        preflight: bool,
    },
    List(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Content {
    Raw {
        raw: String,
        #[serde(default)]
        extension: String,
    },
}

const fn true_by_default() -> bool {
    true
}

fn default_prefix() -> String {
    "tw-".into()
}

impl Default for CorePluginsConfig {
    fn default() -> Self {
        CorePluginsConfig::Map {
            preflight: true_by_default(),
        }
    }
}

use serde::Deserialize;
use swc_atoms::JsWord;
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

    /// TODO(kdy1): Support? Not sure
    #[serde(default)]
    pub plugins: Vec<Option<String>>,

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
    pub screens: AHashMap<JsWord, String>,

    #[serde(default)]
    pub keyframes: AHashMap<JsWord, String>,

    #[serde(default)]
    pub opacity: AHashMap<JsWord, String>,
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

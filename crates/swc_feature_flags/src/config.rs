use std::collections::HashMap;

use serde::Deserialize;

/// Configuration for build-time feature flag transformation
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTimeConfig {
    /// Library configurations: library name -> config
    pub libraries: HashMap<String, LibraryConfig>,

    /// Flags to exclude from build-time marking (one-liners that don't need
    /// DCE)
    #[serde(default)]
    pub exclude_flags: Vec<String>,

    /// Global object name for markers (default: "__SWC_FLAGS__")
    #[serde(default = "default_marker_object")]
    pub marker_object: String,
}

/// Configuration for a single library
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryConfig {
    /// Function names to detect for this library (e.g.,
    /// ["useExperimentalFlags", "getFlags"])
    pub functions: Vec<String>,
}

/// Configuration for runtime feature flag transformation
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeConfig {
    /// Flag values to apply (flag_name -> boolean)
    pub flag_values: HashMap<String, bool>,

    /// Whether to remove markers after processing
    #[serde(default = "default_true")]
    pub remove_markers: bool,

    /// Whether to collect statistics
    #[serde(default = "default_true")]
    pub collect_stats: bool,

    /// Marker object name (must match build-time)
    #[serde(default = "default_marker_object")]
    pub marker_object: String,
}

fn default_marker_object() -> String {
    "__SWC_FLAGS__".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for BuildTimeConfig {
    fn default() -> Self {
        Self {
            libraries: HashMap::new(),
            exclude_flags: Vec::new(),
            marker_object: default_marker_object(),
        }
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            flag_values: HashMap::new(),
            remove_markers: true,
            collect_stats: true,
            marker_object: default_marker_object(),
        }
    }
}

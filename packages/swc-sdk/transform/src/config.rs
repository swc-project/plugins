use default_from_serde::SerdeDefault;
use serde::Deserialize;
use swc_atoms::{Atom, Wtf8Atom};

#[derive(Debug, Clone, Deserialize, SerdeDefault)]
pub struct Config {
    #[serde(default)]
    pub flag: FlagConfig,

    /// Drop imports from the following modules.
    #[serde(default = "default_remove_imports_from")]
    pub remove_imports_from: Vec<Wtf8Atom>,
}

#[derive(Debug, Clone, Deserialize, SerdeDefault)]
pub struct FlagConfig {
    /// If true,
    ///
    /// - the variable name must be an identifier.
    #[serde(default)]
    pub strict: bool,

    #[serde(default = "default_flag_import_source")]
    pub import_sources: Vec<ImportItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImportItem {
    pub module: Atom,
    pub name: Atom,
}

fn default_remove_imports_from() -> Vec<Wtf8Atom> {
    vec![Wtf8Atom::from("@swc/sdk/annotations")]
}

fn default_flag_import_source() -> Vec<ImportItem> {
    vec![ImportItem {
        module: Atom::new("@swc/sdk/flag"),
        name: Atom::new("flag"),
    }]
}

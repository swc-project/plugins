use default_from_serde::SerdeDefault;
use serde::Deserialize;
use swc_atoms::{Atom, Wtf8Atom};

#[derive(Debug, Clone, Deserialize, SerdeDefault)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub flag: FlagConfig,

    #[serde(default)]
    pub dynamic_imports: DynamicImportsConfig,

    /// Drop imports from the following modules.
    #[serde(alias = "remove_imports_from", default = "default_remove_imports_from")]
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

#[derive(Debug, Clone, Deserialize, SerdeDefault)]
#[serde(rename_all = "camelCase")]
pub struct DynamicImportsConfig {
    #[serde(default = "default_import_item_react_lazy")]
    pub lazy_jsx: ImportItem,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportItem {
    pub module: Wtf8Atom,
    pub name: Atom,
}

fn default_remove_imports_from() -> Vec<Wtf8Atom> {
    vec![Wtf8Atom::from("@swc/sdk/annotations")]
}

fn default_flag_import_source() -> Vec<ImportItem> {
    vec![ImportItem {
        module: Wtf8Atom::from("@swc/sdk/flag"),
        name: Atom::new("flag"),
    }]
}

/// `import { lazy } from "react"`
fn default_import_item_react_lazy() -> ImportItem {
    ImportItem {
        module: Wtf8Atom::from("react"),
        name: Atom::new("lazy"),
    }
}

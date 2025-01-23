use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use swc_atoms::JsWord;

use crate::{EmotionModuleConfig, ExportItem};

/// key: `importSource`
pub type ImportMap = FxHashMap<JsWord, ImportMapValue>;

/// key: `localExportName`
pub type ImportMapValue = FxHashMap<JsWord, ImportItemConfig>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ImportItemConfig {
    pub canonical_import: ItemSpecifier,
    pub styled_base_import: Option<ItemSpecifier>,
}

/// `(packageName, exportName)`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSpecifier(pub JsWord, pub JsWord);

pub(crate) fn expand_import_map(
    map: Option<&ImportMap>,
    mut imports: Vec<EmotionModuleConfig>,
) -> Vec<EmotionModuleConfig> {
    if let Some(map) = map {
        map.iter().for_each(|(import_source, value)| {
            value.iter().for_each(
                |(
                    local_export_name,
                    ImportItemConfig {
                        canonical_import, ..
                    },
                )| {
                    let ItemSpecifier(package_name, export_name) = canonical_import;

                    if &**package_name == "@emotion/react" && &**export_name == "jsx" {
                        return;
                    }

                    let package_transformers = imports
                        .iter()
                        .find(|v| v.module_name == *package_name)
                        .unwrap_or_else(|| {
                            panic!(
                                "There is no transformer for the export '{}' in '{}'",
                                export_name, package_name
                            )
                        })
                        .clone();

                    let kind = package_transformers
                        .exported_names
                        .iter()
                        .find(|v| v.name == **export_name)
                        .map(|v| v.kind)
                        .or_else(|| {
                            if export_name == "default" {
                                package_transformers.default_export
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| {
                            panic!(
                                "failed to find export '{}' from package '{}'",
                                export_name, package_name
                            )
                        });

                    imports.push(EmotionModuleConfig {
                        module_name: import_source.clone(),
                        exported_names: vec![ExportItem {
                            name: local_export_name.to_string(),
                            kind,
                        }],
                        default_export: package_transformers.default_export,
                    });
                },
            )
        });
    }

    imports
}

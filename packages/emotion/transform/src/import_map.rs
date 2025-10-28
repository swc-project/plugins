use std::sync::Arc;

use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use swc_atoms::{Atom, Wtf8Atom};

use crate::{EmotionModuleConfig, ExportItem, ExprKind};

/// key: `importSource`
pub type ImportMap = FxHashMap<Wtf8Atom, ImportMapValue>;

/// key: `localExportName`
pub type ImportMapValue = FxHashMap<Atom, ImportItemConfig>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ImportItemConfig {
    pub canonical_import: ItemSpecifier,
    pub styled_base_import: Option<ItemSpecifier>,
}

/// `(packageName, exportName)`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSpecifier(pub Wtf8Atom, pub Atom);

static EMOTION_OFFICIAL_LIBRARIES: Lazy<Arc<Vec<EmotionModuleConfig>>> = Lazy::new(|| {
    Arc::new(vec![
        EmotionModuleConfig {
            module_name: "@emotion/css".into(),
            exported_names: vec![ExportItem {
                name: "css".to_owned(),
                kind: ExprKind::Css,
            }],
            default_export: Some(ExprKind::Css),
        },
        EmotionModuleConfig {
            module_name: "@emotion/styled".into(),
            exported_names: vec![],
            default_export: Some(ExprKind::Styled),
        },
        EmotionModuleConfig {
            module_name: "@emotion/react".into(),
            exported_names: vec![
                ExportItem {
                    name: "css".to_owned(),
                    kind: ExprKind::Css,
                },
                ExportItem {
                    name: "keyframes".to_owned(),
                    kind: ExprKind::Css,
                },
                ExportItem {
                    name: "Global".to_owned(),
                    kind: ExprKind::GlobalJSX,
                },
            ],
            ..Default::default()
        },
        EmotionModuleConfig {
            module_name: "@emotion/primitives".into(),
            exported_names: vec![ExportItem {
                name: "css".to_owned(),
                kind: ExprKind::Css,
            }],
            default_export: Some(ExprKind::Styled),
        },
        EmotionModuleConfig {
            module_name: "@emotion/native".into(),
            exported_names: vec![ExportItem {
                name: "css".to_owned(),
                kind: ExprKind::Css,
            }],
            default_export: Some(ExprKind::Styled),
        },
    ])
});

pub(crate) fn expand_import_map(map: Option<&ImportMap>) -> Arc<Vec<EmotionModuleConfig>> {
    if map.is_none() {
        return EMOTION_OFFICIAL_LIBRARIES.clone();
    }

    let mut imports = EMOTION_OFFICIAL_LIBRARIES.to_vec();

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
                                "There is no transformer for the export '{export_name}' in \
                                 '{package_name:?}'"
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
                                "failed to find export '{export_name}' from package \
                                 '{package_name:?}'"
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

    Arc::new(imports)
}

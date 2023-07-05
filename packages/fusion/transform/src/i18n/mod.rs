use std::collections::HashSet;

use tracing::debug;

pub use self::{
    analyze_imports::i18n_analyze_imports, analyze_use_translation::i18n_analyze_use_translation,
};

mod analyze_imports;
mod analyze_use_translation;

/// This is created once per file.
#[derive(Debug, Default)]
pub struct State {
    translation_ids: HashSet<String>,
    fusion_plugin_imports: HashSet<String>,
    use_translation_alias: HashSet<String>,
}

impl State {
    pub(crate) fn add_translation_id(&mut self, id: String) {
        debug!("adding translation ID {:?}", id);
        self.translation_ids.insert(id);
    }

    pub(crate) fn get_translation_ids(&self) -> &HashSet<String> {
        &self.translation_ids
    }

    pub(crate) fn add_fusion_plugin_import(&mut self, id: String) {
        debug!("adding fusion plugin import {:?}", id);
        self.fusion_plugin_imports.insert(id);
    }

    pub(crate) fn get_fusion_plugin_imports(&self) -> &HashSet<String> {
        &self.fusion_plugin_imports
    }

    pub(crate) fn add_use_translation_alias(&mut self, id: String) {
        debug!("adding use translation alias {:?}", id);
        self.use_translation_alias.insert(id);
    }

    pub(crate) fn get_use_translation_alias(&self) -> &HashSet<String> {
        &self.use_translation_alias
    }
}

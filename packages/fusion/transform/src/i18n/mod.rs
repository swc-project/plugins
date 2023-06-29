use std::collections::HashSet;

use tracing::debug;

pub use self::analyzer::analyzer;

mod analyzer;

/// This is created once per file.
#[derive(Debug, Default)]
pub struct State {
    translation_ids: HashSet<String>,
    fusion_plugin_imports: HashSet<String>,
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

}

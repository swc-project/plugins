pub use crate::utils::{analyze, analyzer, State};
use crate::visitors::{
    display_name_and_id::display_name_and_id, transpile_css_prop::transpile::transpile_css_prop,
};
use serde::Deserialize;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use swc_common::{chain, SourceFile};
use swc_ecmascript::visit::{Fold, VisitMut};

mod css;
mod utils;
mod visitors;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub display_name: Option<String>,

    #[serde(default)]
    pub ssr: bool,

    #[serde(default)]
    pub file_name: bool,

    #[serde(default)]
    pub namespace: String,

    #[serde(default)]
    pub transpile_template_literals: bool,
}

impl Config {
    pub(crate) fn use_namespace(&self) -> String {
        if self.namespace.is_empty() {
            return String::new();
        }
        format!("{}__", self.namespace)
    }
}

pub fn styled_components(file: Arc<SourceFile>, config: Config) -> impl Fold + VisitMut {
    let state: Rc<RefCell<State>> = Default::default();
    let config = Rc::new(config);

    chain!(
        analyzer(state.clone()),
        display_name_and_id(file.clone(), config.clone(), state.clone()),
        transpile_css_prop()
    )
}

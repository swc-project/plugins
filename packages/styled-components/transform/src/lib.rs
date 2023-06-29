#![deny(unused)]

use std::{cell::RefCell, rc::Rc};

use serde::Deserialize;
use swc_core::{
    common::{chain, pass::Optional, FileName},
    ecma::{
        atoms::JsWord,
        visit::{Fold, VisitMut},
    },
};

pub use crate::{
    utils::{analyze, analyzer, State},
    visitors::{
        display_name_and_id::display_name_and_id, transpile_css_prop::transpile::transpile_css_prop,
    },
};

mod css;
mod utils;
mod visitors;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    #[serde(default = "true_by_default")]
    pub display_name: bool,

    #[serde(default = "true_by_default")]
    pub ssr: bool,

    #[serde(default = "true_by_default")]
    pub file_name: bool,

    #[serde(default = "default_index_file_name")]
    pub meaningless_file_names: Vec<String>,

    #[serde(default)]
    pub namespace: String,

    #[serde(default)]
    pub top_level_import_paths: Vec<JsWord>,

    #[serde(default)]
    pub transpile_template_literals: bool,

    #[serde(default)]
    pub minify: bool,

    #[serde(default)]
    pub pure: bool,

    #[serde(default = "true_by_default")]
    pub css_prop: bool,
}

fn true_by_default() -> bool {
    true
}

fn default_index_file_name() -> Vec<String> {
    vec!["index".to_string()]
}

impl Config {
    pub(crate) fn use_namespace(&self) -> String {
        if self.namespace.is_empty() {
            return String::new();
        }
        format!("{}__", self.namespace)
    }
}

/// NOTE: **This is not complete**.
///
/// Only [analyzer] and [display_name_and_id] is implemented.
pub fn styled_components(
    file_name: FileName,
    src_file_hash: u128,
    config: Config,
) -> impl Fold + VisitMut {
    let state: Rc<RefCell<State>> = Default::default();
    let config = Rc::new(config);

    chain!(
        analyzer(config.clone(), state.clone()),
        Optional {
            enabled: config.css_prop,
            visitor: transpile_css_prop(state.clone())
        },
        display_name_and_id(file_name, src_file_hash, config.clone(), state)
    )
}

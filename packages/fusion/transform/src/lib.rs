#![deny(unused)]

use std::{cell::RefCell, rc::Rc};

use serde::Deserialize;
use swc_core::{
    common::{chain, FileName},
    ecma::{
        atoms::JsWord,
        visit::{Fold, VisitMut},
    },
};

pub use crate::{
    asseturl_utils::{analyzer as asseturlAnalyzer, State as asseturlState},
    gql_utils::{analyzer as gqlAnalyzer, State as gqlState},
    i18n::{i18n_analyze_imports, i18n_analyze_use_translation, State as i18n_state},
    visitors::{asseturl::asseturl, dirname::dirname, gql::gql, i18n::i18n_report_ids},
};

mod asseturl_utils;
mod gql_utils;
mod i18n;
mod shared;
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

impl Config {}

pub fn i18n_macro(file_name: FileName) -> impl Fold + VisitMut {
    let state: Rc<RefCell<i18n_state>> = Default::default();

    chain!(
        i18n_analyze_imports(state.clone()),
        i18n_analyze_use_translation(state.clone()),
        i18n_report_ids(file_name.clone(), state)
    )
}

pub fn asseturl_macro(config: Config) -> impl Fold + VisitMut {
    let state: Rc<RefCell<asseturlState>> = Default::default();
    let config = Rc::new(config);

    chain!(
        asseturlAnalyzer(config.clone(), state.clone()),
        asseturl(state)
    )
}

pub fn gql_macro(config: Config) -> impl Fold + VisitMut {
    let state: Rc<RefCell<gqlState>> = Default::default();
    let config = Rc::new(config);

    chain!(gqlAnalyzer(config.clone(), state.clone()), gql(state))
}

pub fn dirname_macro(file_name: FileName) -> impl Fold + VisitMut {
    // let state: Rc<RefCell<gqlState>> = Default::default();
    // let config = Rc::new(config);

    dirname(file_name)
}

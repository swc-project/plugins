#![deny(unused)]

use serde::Deserialize;
use swc_atoms::Wtf8Atom;
use swc_common::{comments::Comments, pass::Optional};
use swc_ecma_ast::{fn_pass, Pass};

pub use crate::{
    utils::{analyze, analyzer, State},
    visitors::{
        css_namespace::css_namespace, display_name_and_id::display_name_and_id,
        minify::visitor::minify, pure_annotation::pure_annotation,
        template_literals::template_literals, transpile_css_prop::transpile::transpile_css_prop,
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
    pub css_namespace: Option<String>,

    #[serde(default)]
    pub top_level_import_paths: Vec<Wtf8Atom>,

    #[serde(default = "true_by_default")]
    pub transpile_template_literals: bool,

    #[serde(default = "true_by_default")]
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

    pub(crate) fn css_namespace_selector(&self) -> Option<String> {
        let css_namespace = self.css_namespace.as_deref()?.trim();

        if css_namespace.is_empty() {
            return None;
        }

        if css_namespace.starts_with('&') {
            return Some(css_namespace.to_string());
        }

        let selector = if is_bare_class_name(css_namespace) {
            format!(".{css_namespace}")
        } else {
            css_namespace.to_string()
        };

        Some(format!("{selector} &"))
    }
}

fn is_bare_class_name(value: &str) -> bool {
    value
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
}

pub fn styled_components<'a, C>(
    file_name: Option<&'a str>,
    src_file_hash: u128,
    config: &'a Config,
    comments: C,
) -> impl 'a + Pass
where
    C: 'a + Comments + Clone,
{
    fn_pass(move |program| {
        let mut state = State::default();

        program.mutate(analyzer(config, &mut state));

        if config.css_prop {
            program.mutate(transpile_css_prop(&mut state));
        }

        if !state.need_work() {
            return;
        }

        if let Some(selector) = config.css_namespace_selector() {
            program.mutate(css_namespace(&state, selector));
        }

        program.mutate((
            Optional {
                enabled: config.minify,
                visitor: minify(&state),
            },
            display_name_and_id(file_name, src_file_hash, config, &state),
            Optional {
                enabled: config.transpile_template_literals,
                visitor: template_literals(&state),
            },
            Optional {
                enabled: config.pure,
                visitor: pure_annotation(comments.clone(), &state),
            },
        ));
    })
}

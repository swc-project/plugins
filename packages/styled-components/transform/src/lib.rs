#![deny(unused)]

use std::{cell::RefCell, rc::Rc, sync::Arc};

use serde::Deserialize;
use swc_atoms::JsWord;
use swc_common::{comments::Comments, pass::Optional, FileName};
use swc_ecma_ast::{Pass, Program};

pub use crate::{
    utils::{analyze, analyzer, State},
    visitors::{
        display_name_and_id::display_name_and_id, minify::visitor::minify,
        pure_annotation::pure_annotation, template_literals::template_literals,
        transpile_css_prop::transpile::transpile_css_prop,
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
}

pub fn styled_components<'a, C>(
    file_name: &'a FileName,
    src_file_hash: u128,
    config: &'a Config,
    comments: C,
) -> impl 'a + Pass
where
    C: Comments,
{
    let state: Rc<RefCell<State>> = Default::default();

    (
        analyzer(config.clone(), state.clone()),
        Optional {
            enabled: config.css_prop,
            visitor: transpile_css_prop(state.clone()),
        },
        MayWork {
            pass: (
                Optional {
                    enabled: config.minify,
                    visitor: minify(state.clone()),
                },
                display_name_and_id(file_name, src_file_hash, config.clone(), state.clone()),
                Optional {
                    enabled: config.transpile_template_literals,
                    visitor: template_literals(state.clone()),
                },
                Optional {
                    enabled: config.pure,
                    visitor: pure_annotation(comments, state.clone()),
                },
            ),
            state,
        },
    )
}

struct MayWork<P>
where
    P: Pass,
{
    pass: P,
    state: Rc<RefCell<State>>,
}

impl<P> Pass for MayWork<P>
where
    P: Pass,
{
    fn process(&mut self, p: &mut Program) {
        {
            let state = self.state.borrow();
            if !state.need_work() {
                return;
            }
        }

        self.pass.process(p);
    }
}

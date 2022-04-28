use std::{cell::RefCell, rc::Rc};

use serde::Deserialize;
use styled_components::{analyzer, transpile_css_prop, Config, State};
use swc_common::FileName;
use swc_plugin::{
    ast::{Program, VisitMutWith},
    chain, plugin_transform, TransformPluginProgramMetadata,
};

#[plugin_transform]
fn styled_components(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Config>(&data.plugin_config)
        .expect("invalid config for styled-components");

    let state: Rc<RefCell<State>> = Default::default();

    let ctx = serde_json::from_str::<SwcContext>(&data.transform_context).expect("invalid context");
    let file_name = match ctx.filename {
        Some(s) => FileName::Real(s.into()),
        None => FileName::Anon,
    };

    // TODO: Use correct value
    let src_file_hash = 0;

    let mut pass = styled_components::styled_components(file_name, src_file_hash, config);

    program.visit_mut_with(&mut pass);

    program
}

#[derive(Debug, Deserialize)]
struct SwcContext {
    #[serde(default)]
    filename: Option<String>,
}

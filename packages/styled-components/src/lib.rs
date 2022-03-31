use std::{cell::RefCell, rc::Rc};

use styled_components::{analyzer, transpile_css_prop, Config, State};
use swc_plugin::{
    ast::{Program, VisitMutWith},
    chain, plugin_transform, TransformPluginProgramMetadata,
};

#[plugin_transform]
fn styled_components(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Config>(&data.plugin_config)
        .expect("invalid config for styled-components");

    let state: Rc<RefCell<State>> = Default::default();
    let config = Rc::new(config);

    let mut pass = chain!(
        analyzer(config.clone(), state.clone()),
        transpile_css_prop()
    );

    program.visit_mut_with(&mut pass);

    program
}

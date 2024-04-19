#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![feature(box_patterns)]

use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{
        plugin_transform,
        proxies::{PluginCommentsProxy, TransformPluginProgramMetadata},
    },
};

#[plugin_transform]
fn swc_confidential_plugin(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<swc_confidential::Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for swc-confidential"),
    )
    .expect("invalid config for swc-confidential");

    program.visit_mut_with(&mut swc_confidential::swc_confidential(
        config,
        PluginCommentsProxy,
    ));

    program
}

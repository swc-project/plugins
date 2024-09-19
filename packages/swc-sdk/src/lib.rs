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
fn swc_sdk_plugin(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<swc_sdk::config::Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for swc-sdk"),
    )
    .expect("invalid config for swc-sdk");

    let unresolved_mark = data.unresolved_mark;

    program.visit_mut_with(&mut swc_sdk::swc_sdk(
        swc_sdk::Env { unresolved_mark },
        config,
        PluginCommentsProxy,
    ));

    program
}

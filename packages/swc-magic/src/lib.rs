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
fn swc_magic_plugin(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<swc_magic::Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for swc-magic"),
    )
    .expect("invalid config for swc-magic");

    let unresolved_mark = data.unresolved_mark;

    program.visit_mut_with(&mut swc_magic::swc_magic(
        unresolved_mark,
        config,
        PluginCommentsProxy,
    ));

    program
}

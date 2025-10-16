#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![feature(box_patterns)]

use swc_core::{
    ecma::ast::Program,
    plugin::{
        metadata::TransformPluginMetadataContextKind, plugin_transform,
        proxies::TransformPluginProgramMetadata,
    },
};

#[plugin_transform]
fn swc_inject_imports_plugin(
    mut program: Program,
    data: TransformPluginProgramMetadata,
) -> Program {
    let config = serde_json::from_str::<swc_inject_imports::Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for swc-experimental-inject-imports"),
    )
    .expect("invalid config for swc-experimental-inject-imports");

    let filename = data
        .get_context(&TransformPluginMetadataContextKind::Filename)
        .unwrap()
        .to_string()
        .into();

    program.mutate(swc_inject_imports::swc_inject_imports(filename, config));

    program
}

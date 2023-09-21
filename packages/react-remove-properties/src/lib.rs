#![allow(clippy::not_unsafe_ptr_arg_deref)]
use swc_core::{
    ecma::{ast::Program, visit::FoldWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn swc_plugin(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Option<react_remove_properties::Config>>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for react-remove-properties"),
    )
    .expect("invalid packages")
    .unwrap_or_else(|| react_remove_properties::Config::All(true));

    program.fold_with(&mut react_remove_properties::react_remove_properties(
        config,
    ))
}

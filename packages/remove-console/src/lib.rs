#![allow(clippy::not_unsafe_ptr_arg_deref)]
use swc_common::SyntaxContext;
use swc_core::{
    ecma::{ast::Program, visit::FoldWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn swc_plugin(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Option<remove_console::Config>>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for remove-console"),
    )
    .expect("invalid packages")
    .unwrap_or_else(|| remove_console::Config::All(true));

    program.fold_with(&mut remove_console::remove_console(
        config,
        SyntaxContext::empty().apply_mark(data.unresolved_mark),
    ))
}

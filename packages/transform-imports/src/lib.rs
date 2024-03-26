#![allow(clippy::not_unsafe_ptr_arg_deref)]
use swc_core::{
    ecma::{ast::Program, visit::FoldWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn transform_imports_plugin(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let packages = serde_json::from_str(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for transform-imports"),
    )
    .expect("invalid packages");

    program.fold_with(&mut modularize_imports::modularize_imports(
        modularize_imports::Config { packages },
    ))
}

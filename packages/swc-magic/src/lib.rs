#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![feature(box_patterns)]

use swc_core::{
    common::{sync::Lrc, FileName, SourceMap},
    ecma::{ast::Program, visit::FoldWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn swc_magic_plugin(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for swc-magic"),
    )
    .expect("invalid config for swc-magic");

    // TODO(kdy1): This is wrong, but it does not use cm
    let cm = Lrc::new(SourceMap::default());

    program.fold_with(&mut visitor::swc_magic(cm, FileName::Anon, config))
}

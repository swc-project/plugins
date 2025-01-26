#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![feature(box_patterns)]

use styled_jsx::{visitor, visitor::Config};
use swc_core::{
    common::{sync::Lrc, FileName, SourceMap},
    ecma::ast::Program,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn styled_jsx_plugin(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for styled-jsx"),
    )
    .expect("invalid config for styled-jsx");

    // TODO(kdy1): This is wrong, but it does not use cm
    let cm = Lrc::new(SourceMap::default());

    program.apply(visitor::styled_jsx(
        cm,
        &FileName::Anon,
        &config,
        &Default::default(),
    ))
}

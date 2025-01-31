#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::sync::Arc;

use serde::Deserialize;
use swc_common::FileName;
use swc_core::{
    ecma::ast::Program,
    plugin::{
        metadata::TransformPluginMetadataContextKind, plugin_transform,
        proxies::TransformPluginProgramMetadata,
    },
};

#[derive(Deserialize)]
struct Config {
    pub transform_code: String,
}

#[plugin_transform]
fn swc_plugin(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for experimental-babel"),
    )
    .expect("invalid config for experimental-babel");

    let file_name = match data.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(v) => FileName::Real(v.into()),
        None => FileName::Anon,
    };

    program.apply(swc_experimental_babel::Transform {
        transform_code: config.transform_code.as_str(),
        cm: Arc::new(data.source_map),
        filename: Arc::new(file_name),
    })
}

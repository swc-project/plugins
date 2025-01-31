#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::sync::Arc;

use swc_common::FileName;
use swc_core::{
    ecma::ast::Program,
    plugin::{
        metadata::TransformPluginMetadataContextKind, plugin_transform,
        proxies::TransformPluginProgramMetadata,
    },
};

const TRANSFORM_CODE: &str = include_str!("../dist/main.js");

#[plugin_transform]
fn swc_plugin(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let file_name = match data.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(v) => FileName::Real(v.into()),
        None => FileName::Anon,
    };

    program.apply(swc_experimental_babel::Transform {
        transform_code: TRANSFORM_CODE,
        cm: Arc::new(data.source_map),
        filename: Arc::new(file_name),
    })
}

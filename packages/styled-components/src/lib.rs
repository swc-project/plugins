#![allow(clippy::not_unsafe_ptr_arg_deref)]

use styled_components::Config;
use swc_common::{SourceMapper, Spanned};
use swc_core::{
    common::FileName,
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{
        metadata::TransformPluginMetadataContextKind, plugin_transform,
        proxies::TransformPluginProgramMetadata,
    },
};

#[plugin_transform]
fn styled_components(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for styled-components"),
    )
    .expect("invalid config for styled-components");

    let file_name = match data.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(s) => FileName::Real(s.into()),
        None => FileName::Anon,
    };

    let pos = data.source_map.lookup_char_pos(program.span().lo);
    let hash = pos.file.src_hash;

    let mut pass = styled_components::styled_components(file_name, hash, config);

    program.visit_mut_with(&mut pass);

    program
}

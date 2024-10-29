#![allow(clippy::not_unsafe_ptr_arg_deref)]

use swc_common::{SourceMapper, Spanned};
use swc_core::{
    ecma::ast::Program,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
#[plugin_transform]
fn swc_plugin(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Option<swc_prefresh::PrefreshPluginConfig>>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for prefresh plugin"),
    )
    .expect("invalid packages")
    .unwrap_or_default();

    let source_map = std::sync::Arc::new(data.source_map);
    let pos = source_map.lookup_char_pos(program.span().lo);
    let hash = format!("{:x}", pos.file.src_hash);

    program.apply(swc_prefresh::swc_prefresh(config, hash))
}

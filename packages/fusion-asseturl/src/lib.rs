#![allow(clippy::not_unsafe_ptr_arg_deref)]

use fusion_asseturl::Config;
use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{
        plugin_transform,
        proxies::TransformPluginProgramMetadata,
    },
};

#[plugin_transform]
fn fusion_asseturl(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for fusion-asseturl"),
    )
    .expect("invalid config for fusion-asseturl");

    let mut pass = fusion_asseturl::asseturl_macro(
        // file_name,
        config,
    );

    program.visit_mut_with(&mut pass);

    program
}

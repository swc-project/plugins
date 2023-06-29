#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::path::PathBuf;

use fusion::Config;
use swc_common::{plugin::metadata::TransformPluginMetadataContextKind, FileName};
use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
use tracing::debug;

#[plugin_transform]
fn fusion_asseturl(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for fusion-asseturl"),
    )
    .expect("invalid config for fusion-asseturl");

    let mut pass = fusion::asseturl_macro(config);

    program.visit_mut_with(&mut pass);

    let config = serde_json::from_str::<Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for fusion-gql"),
    )
    .expect("invalid config for fusion-gql");

    let mut pass = fusion::gql_macro(config);

    let file_name =
        if let Some(filename) = data.get_context(&TransformPluginMetadataContextKind::Filename) {
            FileName::Real(PathBuf::from(filename))
        } else {
            FileName::Anon
        };

    program.visit_mut_with(&mut pass);

    let mut pass = fusion::dirname_macro(file_name.clone());

    program.visit_mut_with(&mut pass);

    debug!("Running i18n macro");

    let mut pass = fusion::i18n_macro(file_name.clone());

    program.visit_mut_with(&mut pass);

    program
}

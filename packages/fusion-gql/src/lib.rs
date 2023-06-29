#![allow(clippy::not_unsafe_ptr_arg_deref)]

use fusion_gql::Config;
use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn fusion_gql(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for fusion-gql"),
    )
    .expect("invalid config for fusion-gql");

    let mut pass = fusion_gql::gql_macro(
        // file_name,
        config,
    );

    program.visit_mut_with(&mut pass);

    program
}

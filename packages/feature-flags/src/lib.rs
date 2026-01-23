#![allow(clippy::not_unsafe_ptr_arg_deref)]

use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
use swc_feature_flags::{BuildTimeConfig, BuildTimeTransform};

/// SWC plugin entry point for feature flag transformation
///
/// This plugin performs build-time marking of feature flags by:
/// - Tracking imports from configured libraries
/// - Detecting destructuring from configured flag functions
/// - Replacing flag identifiers with `__SWC_FLAGS__.flagName`
/// - Removing import statements and hook calls
#[plugin_transform]
fn swc_plugin_feature_flags(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<BuildTimeConfig>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config"),
    )
    .expect("invalid config");

    let mut transform = BuildTimeTransform::new(config);
    program.visit_mut_with(&mut transform);

    program
}

#![allow(clippy::not_unsafe_ptr_arg_deref)]

use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
use swc_feature_flags::{
    BuildTimeConfig, BuildTimeTransform, FeatureFlagsConfig, RuntimeConfig, RuntimeTransform,
    TransformMode,
};

fn validate_feature_flags_config(config: &FeatureFlagsConfig) {
    match config.mode {
        TransformMode::Mark => {
            if config.libraries.is_empty() {
                panic!("FeatureFlagsConfig: \"libraries\" is required in mark mode");
            }
        }
        TransformMode::Shake => {
            if config.flag_values.is_empty() {
                panic!("FeatureFlagsConfig: \"flagValues\" is required in shake mode");
            }
        }
    }

    for (library, library_config) in &config.libraries {
        if library_config.functions.is_empty() {
            panic!(
                "FeatureFlagsConfig: \"functions\" must not be empty for library \"{}\"",
                library
            );
        }
    }
}

fn validate_build_time_config(config: &BuildTimeConfig) {
    if config.libraries.is_empty() {
        panic!("BuildTimeConfig: \"libraries\" is required");
    }

    for (library, library_config) in &config.libraries {
        if library_config.functions.is_empty() {
            panic!(
                "BuildTimeConfig: \"functions\" must not be empty for library \"{}\"",
                library
            );
        }
    }
}

/// SWC plugin entry point for feature flag transformation
///
/// This plugin supports two modes:
/// - **Mark mode** (default): Marks flags with `__SWC_FLAGS__` markers for
///   later substitution. This is phase 1 of a two-phase transformation.
/// - **Shake mode**: Substitutes `__SWC_FLAGS__` markers with boolean values
///   and performs DCE (dead code elimination). This is phase 2.
///
/// The plugin will first try to parse as FeatureFlagsConfig (new unified
/// config), and fall back to BuildTimeConfig (legacy config) for backward
/// compatibility.
#[plugin_transform]
fn swc_plugin_feature_flags(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config_str = &data
        .get_transform_plugin_config()
        .expect("failed to get plugin config");

    // Try to parse as new unified config first
    if let Ok(config) = serde_json::from_str::<FeatureFlagsConfig>(config_str) {
        validate_feature_flags_config(&config);
        match config.mode {
            TransformMode::Mark => {
                // Phase 1: Mark flags with __SWC_FLAGS__ markers
                let build_config = BuildTimeConfig {
                    libraries: config.libraries,
                    marker_object: config.marker_object,
                };
                let mut transform = BuildTimeTransform::new(build_config);
                program.visit_mut_with(&mut transform);
            }
            TransformMode::Shake => {
                // Phase 2: Substitute markers and perform DCE
                let runtime_config = RuntimeConfig {
                    flag_values: config.flag_values,
                    remove_markers: true,
                    collect_stats: config.collect_stats,
                    marker_object: config.marker_object,
                };
                let mut transform = RuntimeTransform::new(runtime_config);
                program.visit_mut_with(&mut transform);
            }
        }
    } else {
        // Fall back to old BuildTimeConfig for backward compatibility
        let config = serde_json::from_str::<BuildTimeConfig>(config_str)
            .expect("invalid config: must be either FeatureFlagsConfig or BuildTimeConfig");
        validate_build_time_config(&config);

        let mut transform = BuildTimeTransform::new(config);
        program.visit_mut_with(&mut transform);
    }

    program
}

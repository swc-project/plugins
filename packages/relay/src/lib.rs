use std::path::PathBuf;

use serde::Deserialize;
use swc_common::{plugin::metadata::TransformPluginMetadataContextKind, FileName};
use swc_core::{
    ecma::{ast::Program, visit::FoldWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
use swc_relay::{relay, Config, OutputFileExtension, ProjectConfig, RelayLanguageConfig};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct WasmConfig {
    root_dir: PathBuf,

    #[serde(default)]
    artifact_directory: Option<PathBuf>,

    #[serde(default)]
    projects: Vec<ProjectConfig>,

    #[serde(default)]
    language: RelayLanguageConfig,

    #[serde(default)]
    output_file_extension: OutputFileExtension,

    #[serde(default)]
    eager_es_modules: bool,
}

#[plugin_transform]
fn relay_plugin_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let filename = if let Some(filename) =
        metadata.get_context(&TransformPluginMetadataContextKind::Filename)
    {
        FileName::Real(PathBuf::from(filename))
    } else {
        FileName::Anon
    };

    let plugin_config: WasmConfig = serde_json::from_str(
        &metadata
            .get_transform_plugin_config()
            .expect("failed to get plugin config for relay"),
    )
    .expect("Should provide plugin config");

    // Unlike native env, we can't use env::current_dir
    // as well as `/cwd` alias. current_dir cannot resolve to actual path,
    // `/cwd` alias won't expand to `real` path but only gives access to the cwd as
    // mounted path, which we can't use in this case.
    let root_dir = plugin_config.root_dir;

    let config = Config {
        projects: plugin_config.projects,
        artifact_directory: plugin_config.artifact_directory,
        language: plugin_config.language,
        eager_es_modules: plugin_config.eager_es_modules,
        output_file_extension: plugin_config.output_file_extension,
    };

    let mut relay = relay(
        config.into(),
        filename,
        root_dir,
        None,
        Some(metadata.unresolved_mark),
    );

    program.apply(relay)
}

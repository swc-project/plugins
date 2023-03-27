use std::path::PathBuf;

use serde_json::Value;
use swc_common::{plugin::metadata::TransformPluginMetadataContextKind, FileName};
use swc_core::{
    ecma::{ast::Program, visit::FoldWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
use swc_relay::{relay, Config, RelayLanguageConfig};

#[plugin_transform]
fn relay_plugin_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let filename = if let Some(filename) =
        metadata.get_context(&TransformPluginMetadataContextKind::Filename)
    {
        FileName::Real(PathBuf::from(filename))
    } else {
        FileName::Anon
    };

    let plugin_config: Value = serde_json::from_str(
        &metadata
            .get_transform_plugin_config()
            .expect("failed to get plugin config for relay"),
    )
    .expect("Should provide plugin config");

    // Unlike native env, we can't use env::current_dir
    // as well as `/cwd` alias. current_dir cannot resolve to actual path,
    // `/cwd` alias won't expand to `real` path but only gives access to the cwd as
    // mounted path, which we can't use in this case.
    let root_dir = PathBuf::from(
        plugin_config["rootDir"]
            .as_str()
            .expect("rootDir is expected"),
    );
    let artifact_directory = plugin_config["artifactDirectory"]
        .as_str()
        .map(PathBuf::from);
    let language = plugin_config["language"]
        .as_str()
        .map_or(RelayLanguageConfig::TypeScript, |v| v.try_into().unwrap());
    let eager_es_modules = plugin_config["eagerEsModules"]
        .as_bool()
        .unwrap_or_default();

    let config = Config {
        artifact_directory,
        language,
        eager_es_modules,
    };

    let mut relay = relay(&config, filename, root_dir, None);

    program.fold_with(&mut relay)
}

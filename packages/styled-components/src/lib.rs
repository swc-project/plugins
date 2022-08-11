use styled_components::Config;
use swc_core::{
    ast::Program,
    common::FileName,
    plugin::{
        metadata::TransformPluginMetadataContextKind, plugin_transform,
        proxies::TransformPluginProgramMetadata,
    },
    visit::VisitMutWith,
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

    // TODO: Use correct value
    let src_file_hash = 0;

    let mut pass = styled_components::styled_components(file_name, src_file_hash, config);

    program.visit_mut_with(&mut pass);

    program
}

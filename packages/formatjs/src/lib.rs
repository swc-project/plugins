use swc_core::{
    ecma::{ast::Program, visit::*},
    plugin::{
        metadata::TransformPluginMetadataContextKind, plugin_transform,
        proxies::TransformPluginProgramMetadata,
    },
};
use swc_formatjs_transform::{create_formatjs_visitor, FormatJSPluginOptions};

#[plugin_transform]
pub fn process(mut program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let filename = metadata.get_context(&TransformPluginMetadataContextKind::Filename);
    let filename = filename.as_deref().unwrap_or("unknown.js");

    let plugin_config = metadata.get_transform_plugin_config();
    let plugin_options: FormatJSPluginOptions = if let Some(plugin_config) = plugin_config {
        serde_json::from_str(&plugin_config).unwrap_or_else(|f| {
            println!("Could not deserialize instrumentation option");
            println!("{:#?}", f);
            Default::default()
        })
    } else {
        Default::default()
    };

    let mut visitor = create_formatjs_visitor(
        std::sync::Arc::new(metadata.source_map),
        metadata.comments.as_ref(),
        plugin_options,
        filename,
    );

    program.visit_mut_with(&mut visitor);

    program
}

use swc_core::{
    ast::Program,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
    visit::FoldWith,
};

#[plugin_transform]
fn loadable_components_plugin(program: Program, data: TransformPluginProgramMetadata) -> Program {}

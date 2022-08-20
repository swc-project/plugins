use swc_core::{
    ast::Program,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
    visit::{VisitMut, VisitMutWith},
};

#[plugin_transform]
fn loadable_components_plugin(
    mut program: Program,
    data: TransformPluginProgramMetadata,
) -> Program {
    program.visit_mut_with(&mut Loadable {});

    program
}

struct Loadable {}

impl VisitMut for Loadable {}

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
    program.visit_mut_with(&mut loadable_transform());

    program
}

pub fn loadable_transform() -> impl VisitMut {
    Loadable {}
}

struct Loadable {}

impl VisitMut for Loadable {}

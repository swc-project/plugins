use swc_core::{
    ecma::ast::Program,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn noop(mut program: Program, _: TransformPluginProgramMetadata) -> Program {
    program
}

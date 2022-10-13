#![allow(clippy::not_unsafe_ptr_arg_deref)]
use swc_core::{
    ecma::ast::Program,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn noop(program: Program, _: TransformPluginProgramMetadata) -> Program {
    program
}

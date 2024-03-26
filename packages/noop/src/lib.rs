#![allow(clippy::not_unsafe_ptr_arg_deref)]

use swc_core::plugin::proxies::TransformPluginProgramMetadata;
use swc_ecma_ast::Program;
use swc_plugin_macro::plugin_transform;

#[plugin_transform]
fn plugin(program: Program, _: TransformPluginProgramMetadata) -> Program {
    program
}

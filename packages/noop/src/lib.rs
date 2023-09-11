#![allow(clippy::not_unsafe_ptr_arg_deref)]

use swc_ecma_ast::Program;
use swc_plugin_macro::plugin_transform;

#[plugin_transform]
fn noop(program: Program, _: TransformPluginProgramMetadata) -> Program {
    program
}

#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![feature(box_patterns)]

use debug_jsx::swc_debug_jsx;
use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn swc_debug_jsx_plugin(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let cm = data.source_map;

    program.visit_mut_with(&mut swc_debug_jsx(Box::new(cm)));

    program
}

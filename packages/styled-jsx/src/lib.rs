use swc_ecmascript::{
    ast::Program,
    visit::{FoldWith, VisitMutWith},
};
use swc_plugin::plugin_transform;

#[plugin_transform]
fn emotion(program: Program, _plugin_config: String, _: String) -> Program {
    let program = program.fold_with(&mut imp::emotion());

    program
}

#[path = "../../../vendor/next.js/packages/next-swc/crates/core/src/emotion/mod.rs"]
mod imp;

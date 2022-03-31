use swc_ecmascript::{ast::Program, visit::FoldWith};
use swc_plugin::{plugin_transform, TransformPluginProgramMetadata};

#[plugin_transform]
fn transform_imports(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let program = program.fold_with(&mut imp::modularize_imports(imp::Config {}));

    program
}

#[path = "../../../vendor/next.js/packages/next-swc/crates/core/src/modularize_imports.rs"]
mod imp;

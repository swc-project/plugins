use swc_ecmascript::{ast::Program, visit::FoldWith};
use swc_plugin::{plugin_transform, TransformPluginProgramMetadata};

#[plugin_transform]
fn transform_imports(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let packages = serde_json::from_str(&data.plugin_config).expect("invalid packages");
    let program = program.fold_with(&mut imp::modularize_imports(imp::Config { packages }));

    program
}

#[path = "../../../vendor/next.js/packages/next-swc/crates/modularize_imports/src/lib.rs"]
mod imp;

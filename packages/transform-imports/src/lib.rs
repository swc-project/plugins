use swc_ecmascript::{ast::Program, visit::FoldWith};
use swc_plugin::{metadata::TransformPluginProgramMetadata, plugin_transform};

#[plugin_transform]
fn transform_imports_plugin(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let packages = serde_json::from_str(&data.get_transform_plugin_config().unwrap_or_default())
        .expect("invalid packages");
    let program = program.fold_with(&mut modularize_imports::modularize_imports(
        modularize_imports::Config { packages },
    ));

    program
}

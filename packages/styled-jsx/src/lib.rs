use swc_core::{
    common::{sync::Lrc, FileName, SourceMap},
    ecma::{ast::Program, visit::FoldWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn styled_jsx_plugin(program: Program, _: TransformPluginProgramMetadata) -> Program {
    // TODO(kdy1): This is wrong, but it does not use cm
    let cm = Lrc::new(SourceMap::default());

    let program = program.fold_with(&mut styled_jsx::styled_jsx(cm, FileName::Anon));

    program
}

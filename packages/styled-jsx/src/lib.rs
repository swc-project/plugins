use swc_core::{
    ast::Program,
    common::{sync::Lrc, FileName, SourceMap},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
    visit::FoldWith,
};

#[plugin_transform]
fn styled_jsx_plugin(program: Program, _: TransformPluginProgramMetadata) -> Program {
    // TODO(kdy1): This is wrong, but it does not use cm
    let cm = Lrc::new(SourceMap::default());

    let program = program.fold_with(&mut styled_jsx::styled_jsx(cm, FileName::Anon));

    program
}

use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecmascript::{ast::Program, visit::FoldWith};
use swc_plugin::{metadata::TransformPluginProgramMetadata, plugin_transform};

#[plugin_transform]
fn styled_jsx_plugin(program: Program, _: TransformPluginProgramMetadata) -> Program {
    // TODO(kdy1): This is wrong, but it does not use cm
    let cm = Lrc::new(SourceMap::default());

    let program = program.fold_with(&mut styled_jsx::styled_jsx(cm, FileName::Anon));

    program
}

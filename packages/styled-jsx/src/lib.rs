use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecmascript::{ast::Program, visit::FoldWith};
use swc_plugin::plugin_transform;

#[plugin_transform]
fn styled_jsx(program: Program, _plugin_config: String, _: String) -> Program {
    // TODO(kdy1): This is wrong, but it does not use cm
    let cm = Lrc::new(SourceMap::default());

    let program = program.fold_with(&mut imp::styled_jsx(cm, FileName::Anon));

    program
}

#[path = "../../../vendor/next.js/packages/next-swc/crates/core/src/styled_jsx/mod.rs"]
mod imp;

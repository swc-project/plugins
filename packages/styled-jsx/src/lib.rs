#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![feature(box_patterns)]

use styled_jsx::visitor;
use swc_core::{
    common::{sync::Lrc, FileName, SourceMap},
    ecma::{ast::Program, visit::FoldWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn styled_jsx_plugin(program: Program, _: TransformPluginProgramMetadata) -> Program {
    // TODO(kdy1): This is wrong, but it does not use cm
    let cm = Lrc::new(SourceMap::default());

    let program = program.fold_with(&mut visitor::styled_jsx(cm, FileName::Anon));

    program
}

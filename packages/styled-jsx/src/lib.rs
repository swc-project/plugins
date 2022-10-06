#![feature(box_patterns)]
use swc_core::{
    common::{sync::Lrc, FileName, SourceMap},
    ecma::{ast::Program, visit::FoldWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

pub mod style;
mod transform_css;
mod utils;
pub mod visitor;

#[plugin_transform]
fn styled_jsx_plugin(program: Program, _: TransformPluginProgramMetadata) -> Program {
    // TODO(kdy1): This is wrong, but it does not use cm
    let cm = Lrc::new(SourceMap::default());

    let program = program.fold_with(&mut visitor::styled_jsx(cm, FileName::Anon));

    program
}

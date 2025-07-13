use std::path::PathBuf;

use swc_core::{
    common::Mark,
    ecma::{
        ast::Pass,
        parser::{EsSyntax, Syntax},
        transforms::{
            base::resolver,
            testing::{test, test_fixture},
        },
        visit::visit_mut_pass,
    },
};
use swc_mut_cjs_exports::TransformVisitor;

fn tr() -> impl Pass {
    let unresolved_mark = Mark::new();
    let top_level_mark = Mark::new();

    (
        resolver(unresolved_mark, top_level_mark, false),
        visit_mut_pass(TransformVisitor::new(unresolved_mark)),
    )
}

#[testing::fixture("tests/fixture/**/input.js")]
#[testing::fixture("tests/fixture/**/input.jsx")]
fn test(input: PathBuf) {
    let dir = input.parent().unwrap().to_path_buf();
    let jsx = input.extension().unwrap() == "jsx";
    let output = if jsx {
        dir.join("output.jsx")
    } else {
        dir.join("output.js")
    };

    test_fixture(
        Syntax::Es(EsSyntax {
            jsx,
            ..Default::default()
        }),
        &|_| tr(),
        &input,
        &output,
        Default::default(),
    );
}

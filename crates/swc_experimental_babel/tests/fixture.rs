use std::path::PathBuf;

use swc_common::{sync::Lrc, FileName, Mark};
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_testing::test_fixture;

#[testing::fixture("tests/basic/**/input.js")]
fn fixture_basic(input: PathBuf) {
    let output = input
        .parent()
        .expect("should have parent directory")
        .join("output.js");

    test_fixture(
        Default::default(),
        &|tr| {
            (
                resolver(Mark::new(), Mark::new(), false),
                swc_experimental_babel::Transform {
                    transform_code: include_str!("./basic-transform.js"),
                    filename: Lrc::new(FileName::Real(input.clone())),
                    cm: tr.cm.clone(),
                },
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

#[testing::fixture("tests/react-compiler/**/input.js")]
fn fixture_react_compiler(input: PathBuf) {
    let output = input
        .parent()
        .expect("should have parent directory")
        .join("output.js");

    test_fixture(
        swc_ecma_parser::Syntax::Es(swc_ecma_parser::EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        &|tr| {
            (
                resolver(Mark::new(), Mark::new(), false),
                swc_experimental_babel::Transform {
                    transform_code: include_str!(
                        "../../../packages/experimetal-babel-react-compiler/dist/main.js"
                    ),
                    filename: Lrc::new(FileName::Real(input.clone())),
                    cm: tr.cm.clone(),
                },
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

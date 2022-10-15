use std::path::PathBuf;

use styled_jsx::visitor::styled_jsx;
use swc_core::{
    common::{chain, FileName, Mark, Span, DUMMY_SP},
    ecma::{
        parser::{EsConfig, Syntax},
        transforms::{
            base::resolver,
            testing::{test_fixture, FixtureTestConfig},
        },
    },
};
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixture/**/input.js")]
fn styled_jsx_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        syntax(),
        &|t| {
            chain!(
                resolver(Mark::new(), Mark::new(), false),
                styled_jsx(
                    t.cm.clone(),
                    FileName::Real(PathBuf::from("/some-project/src/some-file.js"))
                )
            )
        },
        &input,
        &output,
        Default::default(),
    );

    test_fixture(
        syntax(),
        &|t| {
            // `resolver` uses `Mark` which is stored in a thread-local storage (namely
            // swc_common::GLOBALS), and this loop will make `Mark` to be different from the
            // invocation above.
            //
            // 1000 is used because in future I (kdy1) may optimize logic of resolver.
            for _ in 0..1000 {
                let _mark = Mark::fresh(Mark::root());
            }

            chain!(
                resolver(Mark::new(), Mark::new(), false),
                styled_jsx(
                    t.cm.clone(),
                    FileName::Real(PathBuf::from("/some-project/src/some-file.js"))
                )
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

pub struct DropSpan;
impl swc_core::ecma::visit::VisitMut for DropSpan {
    fn visit_mut_span(&mut self, span: &mut Span) {
        *span = DUMMY_SP
    }
}

#[fixture("tests/errors/**/input.js")]
fn styled_jsx_errors(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    let file_name = match input.to_str().unwrap().contains("ts-with-css-resolve") {
        true => FileName::Real(PathBuf::from("/some-project/src/some-file.ts")),
        false => FileName::Real(PathBuf::from("/some-project/src/some-file.js")),
    };

    test_fixture(
        syntax(),
        &|t| styled_jsx(t.cm.clone(), file_name.clone()),
        &input,
        &output,
        FixtureTestConfig {
            allow_error: true,
            ..Default::default()
        },
    );
}

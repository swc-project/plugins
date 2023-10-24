use std::path::PathBuf;

use swc_common::{chain, FileName, Mark, Span, DUMMY_SP};
use swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_transforms::resolver;
use swc_ecma_transforms_testing::{test_fixture, FixtureTestConfig};
use swc_magic::visitor::swc_magic;
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

fn run(input: PathBuf, use_lightningcss: bool) {
    let output = input.parent().unwrap().join(if use_lightningcss {
        "output.lightningcss.js"
    } else {
        "output.swc.js"
    });

    test_fixture(
        syntax(),
        &|t| {
            chain!(
                resolver(Mark::new(), Mark::new(), false),
                swc_magic(
                    t.cm.clone(),
                    FileName::Real(PathBuf::from("/some-project/src/some-file.js")),
                    swc_magic::visitor::Config { use_lightningcss }
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
                swc_magic(
                    t.cm.clone(),
                    FileName::Real(PathBuf::from("/some-project/src/some-file.js")),
                    swc_magic::visitor::Config { use_lightningcss }
                )
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

#[fixture("tests/fixture/**/input.js")]
fn swc_magic_fixture_lightningcs(input: PathBuf) {
    run(input, true);
}

#[fixture("tests/fixture-swc-only/**/input.js")]
#[fixture("tests/fixture/**/input.js")]
fn swc_magic_fixture_swc(input: PathBuf) {
    run(input, false);
}

pub struct DropSpan;
impl swc_ecma_visit::VisitMut for DropSpan {
    fn visit_mut_span(&mut self, span: &mut Span) {
        *span = DUMMY_SP
    }
}

#[fixture("tests/errors/**/input.js")]
fn swc_magic_errors(input: PathBuf) {
    let file_name = match input.to_str().unwrap().contains("ts-with-css-resolve") {
        true => FileName::Real(PathBuf::from("/some-project/src/some-file.ts")),
        false => FileName::Real(PathBuf::from("/some-project/src/some-file.js")),
    };
    {
        let output = input.parent().unwrap().join("output-swc.js");

        test_fixture(
            syntax(),
            &|t| {
                swc_magic(
                    t.cm.clone(),
                    file_name.clone(),
                    swc_magic::visitor::Config {
                        use_lightningcss: false,
                    },
                )
            },
            &input,
            &output,
            FixtureTestConfig {
                allow_error: true,
                ..Default::default()
            },
        );
    }

    {
        let output = input.parent().unwrap().join("output-lightningcss.js");

        test_fixture(
            syntax(),
            &|t| {
                swc_magic(
                    t.cm.clone(),
                    file_name.clone(),
                    swc_magic::visitor::Config {
                        use_lightningcss: true,
                    },
                )
            },
            &input,
            &output,
            FixtureTestConfig {
                allow_error: true,
                ..Default::default()
            },
        );
    }
}

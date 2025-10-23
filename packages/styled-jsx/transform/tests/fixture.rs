use std::path::PathBuf;

use preset_env_base::Versions;
use styled_jsx::visitor::styled_jsx;
use swc_common::{FileName, Mark, Span, DUMMY_SP};
use swc_ecma_parser::{EsSyntax, Syntax};
use swc_ecma_transforms::resolver;
use swc_ecma_transforms_testing::{test_fixture, FixtureTestConfig};
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    })
}

fn run(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    let browsers = Versions {
        chrome: Some("64".parse().unwrap()),
        edge: Some("79".parse().unwrap()),
        firefox: Some("67".parse().unwrap()),
        opera: Some("51".parse().unwrap()),
        safari: Some("12".parse().unwrap()),

        ..Default::default()
    };

    let file_name = FileName::Real(PathBuf::from("/some-project/src/some-file.js"));
    let config = styled_jsx::visitor::Config { browsers };

    let native_config = Default::default();

    test_fixture(
        syntax(),
        &|t| {
            (
                resolver(Mark::new(), Mark::new(), false),
                styled_jsx(t.cm.clone(), &file_name, &config, &native_config),
            )
        },
        &input,
        &output,
        FixtureTestConfig {
            allow_error: true,
            module: Some(true),
            ..Default::default()
        },
    );
}

#[fixture("tests/fixture/**/input.js")]
fn styled_jsx_fixture_lightningcs(input: PathBuf) {
    run(input);
}

pub struct DropSpan;
impl swc_ecma_visit::VisitMut for DropSpan {
    fn visit_mut_span(&mut self, span: &mut Span) {
        *span = DUMMY_SP
    }
}

#[fixture("tests/errors/**/input.js")]
fn styled_jsx_errors(input: PathBuf) {
    let file_name = match input.to_str().unwrap().contains("ts-with-css-resolve") {
        true => FileName::Real(PathBuf::from("/some-project/src/some-file.ts")),
        false => FileName::Real(PathBuf::from("/some-project/src/some-file.js")),
    };

    {
        let output = input.parent().unwrap().join("output.js");
        let config = styled_jsx::visitor::Config {
            ..Default::default()
        };
        let native_config = Default::default();

        test_fixture(
            syntax(),
            &|t| styled_jsx(t.cm.clone(), &file_name, &config, &native_config),
            &input,
            &output,
            FixtureTestConfig {
                allow_error: true,
                module: Some(true),
                ..Default::default()
            },
        );
    }
}

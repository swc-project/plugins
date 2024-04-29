use std::path::PathBuf;

use anyhow::bail;
use lightningcss::stylesheet::ParserOptions;
use preset_env_base::Versions;
use styled_jsx::visitor::{styled_jsx, NativeConfig};
use swc_common::{chain, FileName, Mark, Span, DUMMY_SP};
use swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_transforms::resolver;
use swc_ecma_transforms_testing::{test_fixture, FixtureTestConfig};
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
            let browsers = Versions {
                chrome: Some("64".parse().unwrap()),
                edge: Some("79".parse().unwrap()),
                firefox: Some("67".parse().unwrap()),
                opera: Some("51".parse().unwrap()),
                safari: Some("12".parse().unwrap()),

                ..Default::default()
            };
            chain!(
                resolver(Mark::new(), Mark::new(), false),
                styled_jsx(
                    t.cm.clone(),
                    FileName::Real(PathBuf::from("/some-project/src/some-file.js")),
                    styled_jsx::visitor::Config {
                        use_lightningcss,
                        browsers,
                    },
                    if use_lightningcss {
                        Default::default()
                    } else {
                        NativeConfig {
                            process_css: Some(Box::new(move |css| {
                                let ss = lightningcss::stylesheet::StyleSheet::parse(
                                    css,
                                    ParserOptions {
                                        error_recovery: true,
                                        ..Default::default()
                                    },
                                );

                                let ss = match ss {
                                    Ok(v) => v,
                                    Err(err) => {
                                        bail!(
                                            "failed to parse css using lightningcss: {}\nCode: {}",
                                            err,
                                            css
                                        )
                                    }
                                };

                                let output =
                                    ss.to_css(lightningcss::stylesheet::PrinterOptions {
                                        minify: true,
                                        source_map: None,
                                        project_root: None,
                                        targets: Default::default(),
                                        analyze_dependencies: None,
                                        pseudo_classes: None,
                                    })?;
                                Ok(output.code)
                            })),
                        }
                    }
                )
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

#[fixture("tests/fixture/**/input.js")]
fn styled_jsx_fixture_lightningcs(input: PathBuf) {
    run(input, true);
}

#[fixture("tests/fixture/**/input.js")]
fn styled_jsx_fixture_swc(input: PathBuf) {
    run(input, false);
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
        let output = input.parent().unwrap().join("output-swc.js");

        test_fixture(
            syntax(),
            &|t| {
                styled_jsx(
                    t.cm.clone(),
                    file_name.clone(),
                    styled_jsx::visitor::Config {
                        use_lightningcss: false,
                        ..Default::default()
                    },
                    Default::default(),
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
                styled_jsx(
                    t.cm.clone(),
                    file_name.clone(),
                    styled_jsx::visitor::Config {
                        use_lightningcss: true,
                        ..Default::default()
                    },
                    Default::default(),
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

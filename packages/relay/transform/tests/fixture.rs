use std::path::PathBuf;

use swc_common::FileName;
use swc_ecma_transforms_testing::test_fixture;
use swc_relay::{relay, Config, OutputFileExtension, RelayLanguageConfig};

#[testing::fixture("tests/fixture/simple/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|_| {
            relay(
                &Config {
                    artifact_directory: None,
                    language: RelayLanguageConfig::TypeScript,
                    eager_es_modules: false,
                    output_file_extension: OutputFileExtension::Undefined,
                },
                FileName::Real("file.js".parse().unwrap()),
                Default::default(),
                None,
                None,
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

#[testing::fixture("tests/fixture/eagerEsModules/**/input.js")]
fn fixture_es_modules(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|_| {
            relay(
                &Config {
                    artifact_directory: None,
                    language: RelayLanguageConfig::TypeScript,
                    eager_es_modules: true,
                    output_file_extension: OutputFileExtension::Undefined,
                },
                FileName::Real("file.js".parse().unwrap()),
                Default::default(),
                None,
                None,
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

#[testing::fixture("tests/fixture/output-file-extension/javascript/**/input.js")]
fn fixture_output_file_extension_javascript(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|_| {
            relay(
                &Config {
                    artifact_directory: None,
                    language: RelayLanguageConfig::TypeScript,
                    eager_es_modules: true,
                    output_file_extension: OutputFileExtension::JavaScript,
                },
                FileName::Real("file.js".parse().unwrap()),
                Default::default(),
                None,
                None,
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

#[testing::fixture("tests/fixture/output-file-extension/typescript/**/input.js")]
fn fixture_output_file_extension_typescript(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|_| {
            relay(
                &Config {
                    artifact_directory: None,
                    language: RelayLanguageConfig::JavaScript,
                    eager_es_modules: true,
                    output_file_extension: OutputFileExtension::TypeScript,
                },
                FileName::Real("file.js".parse().unwrap()),
                Default::default(),
                None,
                None,
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

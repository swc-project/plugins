use std::path::PathBuf;

use swc_common::FileName;
use swc_relay::{relay, Config, RelayLanguageConfig};

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

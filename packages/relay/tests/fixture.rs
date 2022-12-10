use std::path::PathBuf;

use swc_common::FileName;
use swc_core::ecma::{transforms::testing::test_fixture, visit::as_folder};
use swc_plugin_relay::{relay, Config, RelayLanguageConfig};

#[testing::fixture("tests/fixture/simple/**/input.js")]
fn fixture(input: PathBuf) {
    println!("Input {:?}", input);
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|t| relay(
            &Config {
                artifact_directory: None,
                language: RelayLanguageConfig::TypeScript,
                eager_es_modules: false,
            },
            FileName::Real("foo".parse().unwrap()),
            Default::default(),
        ),
        &input,
        &output,
        Default::default(),
    );
}

#[testing::fixture("tests/fixture/eagerEsModules/**/input.js")]
fn fixture_es_modules(input: PathBuf) {
    println!("Input {:?}", input);
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|t| relay(
            &Config {
                artifact_directory: None,
                language: RelayLanguageConfig::TypeScript,
                eager_es_modules: true,
            },
            FileName::Real("foo".parse().unwrap()),
            Default::default(),
        ),
        &input,
        &output,
        Default::default(),
    );
}

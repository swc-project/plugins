#![deny(unused)]

use std::{fs::read_to_string, path::PathBuf};

use fusion::{asseturl_macro, dirname_macro, gql_macro, i18n_macro, Config};
use swc_core::{
    common::{chain, FileName, Mark},
    ecma::{
        parser::{EsConfig, Syntax},
        transforms::{base::resolver, testing::test_fixture},
    },
};

#[testing::fixture("tests/fixtures/**/code.js")]
fn fixture(input: PathBuf) {
    let dir = input.parent().unwrap();
    let config = read_to_string(dir.join("config.json")).expect("failed to read config.json");
    println!("---- Configg -----\n{}", config);
    let config: Config = serde_json::from_str(&config).unwrap();

    test_fixture(
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        }),
        &|_| {
            chain!(
                resolver(Mark::new(), Mark::new(), false),
                asseturl_macro(config.clone()),
                gql_macro(config.clone()),
                dirname_macro(FileName::Real(PathBuf::from("/path/to/file.js"))),
                i18n_macro(FileName::Real(PathBuf::from("/path/to/file.js"))),
            )
        },
        &input,
        &dir.join("output.js"),
        Default::default(),
    )
}

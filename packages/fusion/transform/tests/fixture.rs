#![deny(unused)]

use std::{fs::read_to_string, path::PathBuf};

use fusion::{asseturl_macro, gql_macro, Config, dirname_macro};
use swc_core::{
    common::{chain, Mark, FileName},
    ecma::{
        parser::{EsConfig, Syntax},
        transforms::{base::resolver, testing::test_fixture},
    },
};

#[testing::fixture("tests/fixtures/**/code.js")]
fn fixture(input: PathBuf) {
    let dir = input.parent().unwrap();
    let config = read_to_string(dir.join("config.json")).expect("failed to read config.json");
    println!("---- Config -----\n{}", config);
    let config: Config = serde_json::from_str(&config).unwrap();

    test_fixture(
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        }),
        &|_| {
            //
            // let fm = t.cm.load_file(&input).unwrap();

            chain!(
                resolver(Mark::new(), Mark::new(), false),
                asseturl_macro(config.clone()),
                gql_macro(config.clone()),
                dirname_macro(FileName::Real(PathBuf::from("/path/to/file.js"))),
            )
        },
        &input,
        &dir.join("output.js"),
        Default::default(),
    )
}

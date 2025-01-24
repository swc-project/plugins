#![deny(unused)]

use std::{fs::read_to_string, path::PathBuf};

use styled_components::{styled_components, Config};
use swc_common::{FileName, Mark};
use swc_ecma_parser::{EsSyntax, Syntax};
use swc_ecma_transforms::resolver;
use swc_ecma_transforms_testing::{test_fixture, FixtureTestConfig};

#[testing::fixture("tests/fixtures/**/code.js")]
fn fixture(input: PathBuf) {
    let dir = input.parent().unwrap();
    let config = read_to_string(dir.join("config.json")).expect("failed to read config.json");
    println!("---- Config -----\n{}", config);
    let config: Config = serde_json::from_str(&config).unwrap();

    let file_name = FileName::Real(input.clone());

    test_fixture(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        &|t| {
            //
            let fm = t.cm.load_file(&input).unwrap();

            (
                resolver(Mark::new(), Mark::new(), false),
                styled_components(&file_name, fm.src_hash, &config, t.comments.clone()),
            )
        },
        &input,
        &dir.join("output.js"),
        FixtureTestConfig {
            module: Some(true),
            ..Default::default()
        },
    )
}

#![deny(unused)]

use std::path::PathBuf;
use styled_components::styled_components;
use swc_common::chain;
use swc_ecma_transforms_testing::test_fixture;
use swc_ecmascript::{
    parser::{EsConfig, Syntax},
    transforms::resolver,
};

#[testing::fixture("tests/fixtures/**/code.js")]
fn fixture(input: PathBuf) {
    let dir = input.parent().unwrap();

    test_fixture(
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        }),
        &|_| chain!(resolver(), styled_components()),
        &input,
        &dir.join("output.js"),
    )
}

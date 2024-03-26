use std::path::PathBuf;

use swc_core::ecma::{transforms::testing::test_fixture, visit::as_folder};
use swc_plugin_loadable_components::loadable_transform;

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|t| as_folder(loadable_transform(t.comments.clone())),
        &input,
        &output,
        Default::default(),
    );
}

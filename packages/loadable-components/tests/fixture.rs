use std::path::PathBuf;

use swc_core::{testing_transform::test_fixture, visit::as_folder};
use swc_plugin_loadable_components::loadable_transform;

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("ouput.js");

    test_fixture(
        Default::default(),
        &|_| as_folder(loadable_transform()),
        &input,
        &output,
    );
}

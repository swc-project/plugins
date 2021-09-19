use std::path::PathBuf;
use swc_common::chain;
use swc_ecma_transforms_base::resolver::resolver;
use swc_emotion::emotion_plugin;

#[testing::fixture("tests/fixtures/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    swc_ecma_transforms_testing::test_fixture(
        Default::default(),
        &|_t| chain!(resolver(), emotion_plugin()),
        &input,
        &output,
    );
}

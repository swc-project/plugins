use std::path::PathBuf;

#[testing::fixture("tests/fixture/**/*.css")]
fn transform(input: PathBuf) {}

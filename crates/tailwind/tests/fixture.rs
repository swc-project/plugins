use std::path::PathBuf;

#[testing::fixture("tests/fixture/**/input.css")]
fn transform(input: PathBuf) {}

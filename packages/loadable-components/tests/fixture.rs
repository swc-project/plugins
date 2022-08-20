use std::path::PathBuf;

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {}

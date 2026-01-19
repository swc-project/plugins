use std::path::PathBuf;

use remove_assert::{remove_assert, Config};
use swc_common::SyntaxContext;
use swc_ecma_parser::{EsSyntax, Syntax};
use swc_ecma_transforms_testing::test_fixture;

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input.with_file_name("output.js");
    test_fixture(
        Syntax::Es(EsSyntax::default()),
        &|_tr| remove_assert(Config::All(true), SyntaxContext::empty()),
        &input,
        &output,
        Default::default(),
    );
}

use std::path::PathBuf;

use swc_constify::constify;
use swc_core::{
    common::{chain, Mark},
    ecma::{
        transforms::{base::resolver, testing::test_fixture},
        visit::as_folder,
    },
};

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|_| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            as_folder(chain!(
                resolver(unresolved_mark, top_level_mark, false),
                constify()
            ))
        },
        &input,
        &output,
        Default::default(),
    );
}

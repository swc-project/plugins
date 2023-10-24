use std::path::PathBuf;

use swc_common::Mark;

#[testing::fixture("tests/fixture/**/input.js")]
fn pure(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        syntax(),
        &|tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            chain!(
                resolver(unresolved_mark, top_level_mark, false),
                swc_magic(unresolved_mark, tr.comments.clone(tr.comments.cloen()))
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

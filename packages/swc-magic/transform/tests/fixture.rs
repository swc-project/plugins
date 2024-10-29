use std::path::PathBuf;

use swc_common::{chain, Mark};
use swc_ecma_parser::Syntax;
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_testing::test_fixture;
use swc_ecma_visit::visit_mut_pass;

#[testing::fixture("tests/fixture/**/input.js")]
fn pure(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        Syntax::default(),
        &|tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            (
                resolver(unresolved_mark, top_level_mark, false),
                visit_mut_pass(swc_magic::swc_magic(
                    unresolved_mark,
                    swc_magic::Config {
                        import_path: "@swc/magic".into(),
                    },
                    tr.comments.clone(),
                )),
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

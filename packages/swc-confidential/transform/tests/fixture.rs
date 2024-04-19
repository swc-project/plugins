use std::path::PathBuf;

use swc_common::{chain, Mark};
use swc_ecma_parser::Syntax;
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_testing::test_fixture;
use swc_ecma_visit::as_folder;

#[testing::fixture("tests/fixture/**/input.js")]
fn pure(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        Syntax::default(),
        &|tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            chain!(
                resolver(unresolved_mark, top_level_mark, false),
                as_folder(swc_confidential::swc_confidential(
                    unresolved_mark,
                    swc_confidential::Config {
                        import_path: "@swc/magic".into()
                    },
                    tr.comments.clone()
                ))
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

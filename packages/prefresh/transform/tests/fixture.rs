use std::path::PathBuf;

use swc_common::Mark;
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_testing::test_fixture;

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input
        .parent()
        .expect("should have parent directory")
        .join("output.js");

    test_fixture(
        Default::default(),
        &|_tr| {
            (
                resolver(Mark::new(), Mark::new(), false),
                swc_prefresh::swc_prefresh(
                    swc_prefresh::PrefreshPluginConfig {
                        library: vec![
                            "@custom/preact".to_string(),
                            "preact".to_string(),
                            "react".to_string(),
                        ],
                    },
                    "__file_hash__".to_string(),
                ),
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

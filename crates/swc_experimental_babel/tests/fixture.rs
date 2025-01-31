use std::path::PathBuf;

use swc_common::{sync::Lrc, FileName, Mark};
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_testing::test_fixture;

#[testing::fixture("tests/prefresh/**/input.js")]

fn fixture(input: PathBuf) {
    let output = input
        .parent()
        .expect("should have parent directory")
        .join("output.js");

    test_fixture(
        Default::default(),
        &|tr| {
            (
                resolver(Mark::new(), Mark::new(), false),
                swc_experimental_babel::Transform {
                    transform_code: include_str!("./prefresh-transform.js"),
                    filename: Lrc::new(FileName::Real(input.clone())),
                    cm: tr.cm.clone(),
                },
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

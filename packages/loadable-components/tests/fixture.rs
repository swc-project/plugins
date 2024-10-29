use std::path::PathBuf;

use swc_core::ecma::transforms::testing::test_fixture;
use swc_ecma_visit::visit_mut_pass;
use swc_plugin_loadable_components::{loadable_transform, Signature};

#[testing::fixture("tests/fixture/aggressive import/**/input.js")]
#[testing::fixture("tests/fixture/lazy/**/input.js")]
#[testing::fixture("tests/fixture/loadable.lib/**/input.js")]
#[testing::fixture("tests/fixture/magic comments/**/input.js")]
#[testing::fixture("tests/fixture/simple import/**/input.js")]
fn fixture_default_signatures(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|t| {
            visit_mut_pass(loadable_transform(
                t.comments.clone(),
                vec![Signature::default(), Signature::default_lazy()],
            ))
        },
        &input,
        &output,
        Default::default(),
    );
}

#[testing::fixture("tests/fixture/signatures/**/input.js")]
fn fixture_custom_signatures(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|t| {
            visit_mut_pass(loadable_transform(
                t.comments.clone(),
                vec![
                    Signature {
                        name: "lazy".into(),
                        from: "my-custom-package".into(),
                    },
                    Signature {
                        name: "custom".into(),
                        from: "my-custom-package".into(),
                    },
                    Signature {
                        name: "default".into(),
                        from: "my-custom-package".into(),
                    },
                ],
            ))
        },
        &input,
        &output,
        Default::default(),
    );
}

use std::{collections::HashMap, path::PathBuf};

use swc_common::Mark;
use swc_ecma_parser::{EsSyntax, Syntax};
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_testing::{test_fixture, FixtureTestConfig};
use swc_feature_flags::{
    build_time_pass, runtime_pass, BuildTimeConfig, LibraryConfig, RuntimeConfig,
};

fn syntax() -> Syntax {
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    })
}

#[testing::fixture("tests/fixture/build-time/**/input.js")]
fn build_time_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    let mut libraries = HashMap::new();
    libraries.insert(
        "@their/library".to_string(),
        LibraryConfig {
            functions: vec!["useExperimentalFlags".to_string(), "getFlags".to_string()],
        },
    );

    let config = BuildTimeConfig {
        libraries,
        marker_object: "__SWC_FLAGS__".to_string(),
    };

    test_fixture(
        syntax(),
        &|_tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            (
                resolver(unresolved_mark, top_level_mark, false),
                build_time_pass(config.clone()),
            )
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}

#[testing::fixture("tests/fixture/runtime/**/input.js")]
fn runtime_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    let mut flag_values = HashMap::new();
    flag_values.insert("featureA".to_string(), true);
    flag_values.insert("featureB".to_string(), false);
    flag_values.insert("newCheckout".to_string(), true);
    flag_values.insert("betaFeatures".to_string(), false);

    let config = RuntimeConfig {
        flag_values,
        remove_markers: true,
        collect_stats: true,
        marker_object: "__SWC_FLAGS__".to_string(),
    };

    test_fixture(
        syntax(),
        &|_tr| runtime_pass(config.clone()),
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}

use std::{path::PathBuf, sync::Arc};

use swc_ecma_parser::{EsSyntax, Syntax};
use swc_ecma_transforms_testing::{test_fixture, FixtureTestConfig};
use testing::fixture;
use transform_ns_imports::{transform_ns_imports, PackageConfig};

fn syntax() -> Syntax {
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixture/**/input.js")]
fn transform_ns_imports_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    let config = transform_ns_imports::Config {
        packages: vec![(
            "rewrite-module-namespace".to_string(),
            PackageConfig {
                rewrite: "rewrite-module-namespace/swc-proxy".into(),
            },
        )]
        .into_iter()
        .map(|(k, v)| (k, Arc::new(v)))
        .collect(),
    };

    test_fixture(
        syntax(),
        &|_tr| transform_ns_imports(&config),
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}

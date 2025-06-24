use std::{path::PathBuf, sync::Arc};

use modularize_imports::{modularize_imports, PackageConfig};
use swc_ecma_parser::{EsSyntax, Syntax};
use swc_ecma_transforms_testing::{test_fixture, FixtureTestConfig};
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixture/**/input.js")]
fn modularize_imports_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    let config = modularize_imports::Config {
        packages: vec![
            (
                "react-bootstrap".to_string(),
                PackageConfig {
                    transform: "react-bootstrap/lib/{{member}}".into(),
                    prevent_full_import: false,
                    skip_default_conversion: false,
                    handle_default_import: true,
                    handle_namespace_import: true,
                    rewrite_namespace_to_proxy: false,
                },
            ),
            (
                "my-library/?(((\\w*)?/?)*)".to_string(),
                PackageConfig {
                    transform: "my-library/{{ matches.[1] }}/{{member}}".into(),
                    prevent_full_import: false,
                    skip_default_conversion: false,
                    handle_default_import: true,
                    handle_namespace_import: true,
                    rewrite_namespace_to_proxy: false,
                },
            ),
            (
                "my-library-2".to_string(),
                PackageConfig {
                    transform: "my-library-2/{{ camelCase member }}".into(),
                    prevent_full_import: false,
                    skip_default_conversion: true,
                    handle_default_import: true,
                    handle_namespace_import: true,
                    rewrite_namespace_to_proxy: false
                },
            ),
            (
                "my-library-3".to_string(),
                PackageConfig {
                    transform: "my-library-3/{{ kebabCase member }}".into(),
                    prevent_full_import: false,
                    skip_default_conversion: true,
                    handle_default_import: true,
                    handle_namespace_import: true,
                    rewrite_namespace_to_proxy: false
                },
            ),
            (
                "rewrite-module-namespace".to_string(),
                PackageConfig {
                    transform: "rewrite-module-namespace/{{ member }}".into(),
                    prevent_full_import: false,
                    skip_default_conversion: true,
                    handle_default_import: true,
                    handle_namespace_import: true,
                    rewrite_namespace_to_proxy: true
                },
            ),
            (
                "my-library-4".to_string(),
                PackageConfig {
                    transform: Vec::from([
                        ("foo".to_string(), "my-library-4/this_is_foo".to_string()),
                        ("bar".to_string(), "my-library-4/bar".to_string()),
                        (
                            "use(\\w*)".to_string(),
                            "my-library-4/{{ kebabCase member }}/{{ kebabCase memberMatches.[1] }}"
                                .to_string(),
                        ),
                        (
                            "(\\w*)Icon".to_string(),
                            "my-library-4/{{ kebabCase memberMatches.[1] }}".to_string(),
                        ),
                        (
                            "*".to_string(),
                            "my-library-4/{{ upperCase member }}".to_string(),
                        ),
                    ])
                    .into(),
                    prevent_full_import: false,
                    skip_default_conversion: true,
                    handle_default_import: true,
                    handle_namespace_import: true,
                    rewrite_namespace_to_proxy: false
                },
            ),
            (
                "my-(module-namespace|default|mixed-(named|star))".to_string(),
                PackageConfig {
                    transform: "transformed-{{matches.[1]}}".into(),
                    prevent_full_import: false,
                    skip_default_conversion: true,
                    handle_default_import: true,
                    handle_namespace_import: true,
                    rewrite_namespace_to_proxy: false
                },
            ),
            (
                "^(\\..*)(\\.tsx?)$".to_string(),
                PackageConfig {
                    transform: "{{matches.[1]}}.js".into(),
                    prevent_full_import: false,
                    skip_default_conversion: false,
                    handle_default_import: false,
                    handle_namespace_import: false,
                    rewrite_namespace_to_proxy: false
                },
            ),
        ]
        .into_iter()
        .map(|(k, v)| (k, Arc::new(v)))
        .collect(),
    };

    test_fixture(
        syntax(),
        &|_tr| modularize_imports(&config),
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}

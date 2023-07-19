use std::path::PathBuf;

use modularize_imports::{modularize_imports, PackageConfig};
use swc_core::ecma::{
    parser::{EsConfig, Syntax},
    transforms::testing::{test_fixture, FixtureTestConfig},
};
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixture/**/input.js")]
fn modularize_imports_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        syntax(),
        &|_tr| {
            modularize_imports(modularize_imports::Config {
                packages: vec![
                    (
                        "react-bootstrap".to_string(),
                        PackageConfig {
                            transform: "react-bootstrap/lib/{{member}}".into(),
                            prevent_full_import: false,
                            skip_default_conversion: false,
                        },
                    ),
                    (
                        "my-library/?(((\\w*)?/?)*)".to_string(),
                        PackageConfig {
                            transform: "my-library/{{ matches.[1] }}/{{member}}".into(),
                            prevent_full_import: false,
                            skip_default_conversion: false,
                        },
                    ),
                    (
                        "my-library-2".to_string(),
                        PackageConfig {
                            transform: "my-library-2/{{ camelCase member }}".into(),
                            prevent_full_import: false,
                            skip_default_conversion: true,
                        },
                    ),
                    (
                        "my-library-3".to_string(),
                        PackageConfig {
                            transform: "my-library-3/{{ kebabCase member }}".into(),
                            prevent_full_import: false,
                            skip_default_conversion: true,
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
                                    "my-library-4/{{ kebabCase member }}/{{ kebabCase \
                                     memberMatches.[1] }}"
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
                        },
                    ),
                ]
                .into_iter()
                .collect(),
            })
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}

use std::path::PathBuf;

use swc_atoms::Atom;
use swc_common::Mark;
use swc_ecma_parser::Syntax;
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_testing::test_fixture;
use swc_inject_imports::Config;

#[testing::fixture("tests/fixture/**/input.js")]
fn pure(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        Syntax::default(),
        &|_tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            (
                resolver(unresolved_mark, top_level_mark, false),
                swc_inject_imports::swc_inject_imports(
                    "test.js".into(),
                    Config {
                        imports_paths: vec![Atom::new("@swc/inject-imports")],
                        only_filenames: vec![],
                    },
                ),
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

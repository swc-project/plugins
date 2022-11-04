use std::path::PathBuf;

use swc_core::css::{ast::Stylesheet, parser::parser::ParserConfig};

#[testing::fixture("tests/fixture/**/input.css")]
fn transform(input: PathBuf) {
    let dir = input.parent().unwrap();

    testing::run_test(false, |cm, handler| {
        let fm = cm.load_file(&input).expect("failed to load input css file");

        let mut errors = vec![];
        let ss: Stylesheet = swc_core::css::parser::parse_file(
            &fm,
            ParserConfig {
                ..Default::default()
            },
            &mut errors,
        )
        .expect("failed to parse input css file");

        if !errors.is_empty() {
            for err in errors {
                err.to_diagnostics(&handler).emit();
            }

            return Err(());
        }

        Ok(())
    })
    .unwrap();
}

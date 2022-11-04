use std::path::PathBuf;

use swc_core::css::{ast::Stylesheet, parser::parser::ParserConfig};
use swc_tailwind::Compiler;

#[testing::fixture("tests/fixture/**/input.css")]
fn transform(input: PathBuf) {
    let dir = input.parent().unwrap();
    let config_path = dir.join("config.json");
    let config = serde_json::from_str(&std::fs::read_to_string(&config_path).unwrap()).unwrap();

    testing::run_test(false, |cm, handler| {
        let fm = cm.load_file(&input).expect("failed to load input css file");

        let mut errors = vec![];
        let mut ss: Stylesheet = swc_core::css::parser::parse_file(
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

        let compiler = Compiler::new(config);

        compiler.process(&mut ss);

        Ok(())
    })
    .unwrap();
}

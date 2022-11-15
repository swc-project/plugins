use std::{fs::read_to_string, path::Path};

use swc_common::FileName;
use swc_core::css::{
    codegen::{writer::basic::BasicCssWriter, CodeGenerator, Emit},
    parser::parse_file,
};
use swc_tailwind::Tailwind;

#[test]
fn should_generate_css_using_values_from_your_config_file() {
    let input = read_to_string("tests/fixture/colors/index.css").unwrap();

    let res = run(
        &input,
        "./tests/fixture/colors/tailwind.config.json".as_ref(),
    );

    assert_eq!(
        res.css,
        format_css(
            "
            .text-primary {
                color: #0088cc;
            }
            "
        )
    );
}

fn format_css(s: &str) -> String {
    s.into()
}

struct Output {
    css: String,
}

fn run(input: &str, config_path: &Path) -> Output {
    testing::run_test(false, |cm, _handler| {
        let fm = cm.new_source_file(FileName::Custom("input.css".into()), input.into());

        let mut ss = parse_file(&fm, Default::default(), &mut vec![]).unwrap();

        let mut tw = Tailwind::new(cm, config_path.into());

        tw.compile(&mut ss).expect("failed to compile");

        let css = {
            let mut buf = String::new();
            let mut g = CodeGenerator::new(
                BasicCssWriter::new(&mut buf, None, Default::default()),
                Default::default(),
            );

            g.emit(&ss).unwrap();

            buf
        };

        Ok(Output { css })
    })
    .unwrap()
}

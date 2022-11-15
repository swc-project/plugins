use std::{fs::read_to_string, path::Path};

use swc_common::FileName;
use swc_core::css::{
    ast::Stylesheet,
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

#[test]
fn should_generate_css_using_built_in_plugins() {
    let input = read_to_string("tests/fixture/basic/index.css").unwrap();

    let res = run(
        &input,
        "./tests/fixture/basic/tailwind.config.json".as_ref(),
    );

    assert_eq!(
        res.css,
        format_css(
            "
            .built-in-utility {
                color: red;
            }
            "
        )
    );
}

#[test]
fn should_generate_css_using_static_plugins_defined_in_your_css() {
    let input = read_to_string("tests/fixture/css-plugin/index.css").unwrap();

    let res = run(
        &input,
        "./tests/fixture/css-plugin/tailwind.config.json".as_ref(),
    );

    assert_eq!(
        res.css,
        format_css(
            "
            .css-utility {
                color: blue;
            }
            "
        )
    );
}

fn format_css(s: &str) -> String {
    testing::run_test(false, |cm, _| {
        let fm = cm.new_source_file(FileName::Anon, s.into());

        let ss: Stylesheet = parse_file(&fm, Default::default(), &mut vec![]).unwrap();

        let css = {
            let mut buf = String::new();
            let mut g = CodeGenerator::new(
                BasicCssWriter::new(&mut buf, None, Default::default()),
                Default::default(),
            );

            g.emit(&ss).unwrap();

            buf
        };

        Ok(css)
    })
    .unwrap()
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
        let css = format_css(&css);

        Ok(Output { css })
    })
    .unwrap()
}

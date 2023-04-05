use std::path::PathBuf;

use swc_core::{
    common::{chain, comments::SingleThreadedComments, Mark},
    ecma::{
        parser::{Syntax, TsConfig},
        transforms::{
            react::{jsx, Runtime},
            testing::test_fixture,
        },
    },
};
use swc_emotion::EmotionOptions;
use testing::fixture;

fn ts_syntax() -> Syntax {
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixture/**/input.tsx")]
fn next_emotion_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.ts");
    test_fixture(
        ts_syntax(),
        &|tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();
            let jsx = jsx::<SingleThreadedComments>(
                tr.cm.clone(),
                Some(tr.comments.as_ref().clone()),
                swc_core::ecma::transforms::react::Options {
                    next: false.into(),
                    runtime: Some(Runtime::Automatic),
                    throw_if_namespace: false.into(),
                    development: false.into(),
                    ..Default::default()
                },
                top_level_mark,
                unresolved_mark,
            );

            let test_import_map =
                serde_json::from_str(include_str!("./testImportMap.json")).unwrap();
            let fm = tr.cm.load_file(&input).unwrap();
            chain!(
                swc_emotion::emotion(
                    EmotionOptions {
                        enabled: Some(true),
                        sourcemap: Some(true),
                        auto_label: Some(true),
                        import_map: Some(test_import_map),
                        ..Default::default()
                    },
                    &PathBuf::from("input.ts"),
                    fm.src_hash as u32,
                    tr.cm.clone(),
                    tr.comments.as_ref().clone(),
                ),
                jsx
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

// Test the label format generation behaviour.
// This uses the same input for each output, where each
// output is validating the different labelling options.
//
// Each folder name in `/labels` specifies the label option being tested:
//   "/labels/dirname" -> "[dirname]"
//   "/labels/dirname-filename-local" -> "[dirname]-[filename]-[local]"
//   "/labels/filename" -> "[filename]"
//   "/labels/filename-local" -> "[filename]-[local]"
//   "/labels/local" -> "[local]"
#[fixture("tests/labels/**/output.js")]
fn emotion_label_fixture(output: PathBuf) {
    let output_folder = output.parent().unwrap();
    let output_folder_name = output_folder.file_name().unwrap().to_str().unwrap();
    let input = output_folder.parent().unwrap().join("input.tsx");

    // Simulate the input path for fairly represented maps in the fixture output.
    let mut pseudo_input_path = PathBuf::from(output_folder);
    pseudo_input_path.push("input.tsx");

    let label_option = if output_folder_name.contains('-') {
        // Multiple labelling specifiers, e.g. [filename]-[local]
        output_folder_name
            .split('-')
            .map(|s| format!("[{s}]"))
            .collect::<Vec<String>>()
            .join("-")
    } else {
        // Singular labelling specifiers, e.g. [local]
        format!("[{output_folder_name}]")
    };

    test_fixture(
        ts_syntax(),
        &|tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();
            let jsx = jsx::<SingleThreadedComments>(
                tr.cm.clone(),
                Some(tr.comments.as_ref().clone()),
                swc_core::ecma::transforms::react::Options {
                    next: false.into(),
                    runtime: Some(Runtime::Automatic),
                    throw_if_namespace: false.into(),
                    development: false.into(),
                    ..Default::default()
                },
                top_level_mark,
                unresolved_mark,
            );
            let fm = tr.cm.load_file(&input).unwrap();
            chain!(
                swc_emotion::emotion(
                    EmotionOptions {
                        enabled: Some(true),
                        sourcemap: Some(true),
                        auto_label: Some(true),
                        label_format: Some(label_option.to_string()),
                        ..Default::default()
                    },
                    &PathBuf::from(format!("{output_folder_name}/index.tsx")),
                    fm.src_hash as u32,
                    tr.cm.clone(),
                    tr.comments.as_ref().clone(),
                ),
                jsx
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

#[fixture("tests/label-sanitisation/**/*.ts")]
fn emotion_label_sanitisation(input: PathBuf) {
    let output_folder_name = input
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let input_file_name = input.file_name().unwrap().to_str().unwrap();
    let mut output = PathBuf::from(&input);
    output.set_extension("js");

    test_fixture(
        ts_syntax(),
        &|tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();
            let jsx = jsx::<SingleThreadedComments>(
                tr.cm.clone(),
                Some(tr.comments.as_ref().clone()),
                swc_core::ecma::transforms::react::Options {
                    next: false.into(),
                    runtime: Some(Runtime::Automatic),
                    throw_if_namespace: false.into(),
                    development: false.into(),
                    ..Default::default()
                },
                top_level_mark,
                unresolved_mark,
            );
            let fm = tr.cm.load_file(&input).unwrap();
            chain!(
                swc_emotion::emotion(
                    EmotionOptions {
                        enabled: Some(true),
                        sourcemap: Some(true),
                        auto_label: Some(true),
                        label_format: Some("[dirname]-[filename]-[local]".to_string()),
                        ..Default::default()
                    },
                    &PathBuf::from(format!("{output_folder_name}/{input_file_name}")),
                    fm.src_hash as u32,
                    tr.cm.clone(),
                    tr.comments.as_ref().clone(),
                ),
                jsx
            )
        },
        &input,
        &output,
        Default::default(),
    );
}

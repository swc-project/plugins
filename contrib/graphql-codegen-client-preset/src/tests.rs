use std::path::PathBuf;

use swc_core::{
    ecma::{
        parser::{Syntax, TsSyntax},
        transforms::testing::{test, test_fixture},
        visit::visit_mut_pass,
    },
    testing,
};

use super::*;

fn get_test_code_visitor() -> GraphQLVisitor {
    GraphQLVisitor::new(GraphQLCodegenOptions {
        filename: "test.ts".to_string(),
        cwd: "/home/faketestproject".to_string(),
        artifact_directory: "./src/gql".to_string(),
        gql_tag_name: "gql".to_string(),
        naming_convention: "change-case-all#pascalCase".to_string(),
        document_variable_prefix: "".to_string(),
        document_variable_suffix: "Document".to_string(),
        fragment_variable_prefix: "".to_string(),
        fragment_variable_suffix: "FragmentDoc".to_string(),
        dedupe_operation_suffix: false,
        omit_operation_suffix: false,
    })
}

fn get_test_code_visitor_upper_case_first() -> GraphQLVisitor {
    GraphQLVisitor::new(GraphQLCodegenOptions {
        filename: "test.ts".to_string(),
        cwd: "/home/faketestproject".to_string(),
        artifact_directory: "./src/gql".to_string(),
        gql_tag_name: "gql".to_string(),
        naming_convention: "change-case-all#upperCaseFirst".to_string(),
        document_variable_prefix: "".to_string(),
        document_variable_suffix: "Document".to_string(),
        fragment_variable_prefix: "".to_string(),
        fragment_variable_suffix: "FragmentDoc".to_string(),
        dedupe_operation_suffix: false,
        omit_operation_suffix: false,
    })
}

#[cfg(not(target_os = "windows"))]
#[testing::fixture("tests/fixtures/simple-uppercase-operation-name.ts")]
fn import_files_from_same_directory(input_path: PathBuf) {
    let cwd = std::env::current_dir().unwrap();

    let relative_file_path = diff_paths(&input_path, &cwd).unwrap();

    let output_path = input_path.with_extension("js");

    test_fixture(
        Syntax::Typescript(TsSyntax {
            tsx: input_path.to_string_lossy().ends_with(".tsx"),
            ..Default::default()
        }),
        &|_metadata| {
            visit_mut_pass(GraphQLVisitor::new(GraphQLCodegenOptions {
                filename: relative_file_path.to_string_lossy().to_string(),
                cwd: cwd.to_string_lossy().to_string(),
                artifact_directory: "./tests/fixtures".to_string(),
                gql_tag_name: "gql".to_string(),
                naming_convention: "change-case-all#pascalCase".to_string(),
                document_variable_prefix: "".to_string(),
                document_variable_suffix: "Document".to_string(),
                fragment_variable_prefix: "".to_string(),
                fragment_variable_suffix: "FragmentDoc".to_string(),
                dedupe_operation_suffix: false,
                omit_operation_suffix: false,
            }))
        },
        &input_path,
        &output_path,
        Default::default(),
    );
}

#[cfg(not(target_os = "windows"))]
#[testing::fixture("tests/fixtures/simple-uppercase-operation-name.ts")]
fn import_files_from_other_directory(input_path: PathBuf) {
    // Let's do the same test as for the babel plugin, assume we are in the tests
    // folder
    let mut cwd = std::env::current_dir().unwrap();
    cwd.push("tests");

    let relative_file_path = diff_paths(&input_path, &cwd).unwrap();

    let output_path = input_path.with_extension("other-dir.js");

    test_fixture(
        Syntax::Typescript(TsSyntax {
            tsx: input_path.to_string_lossy().ends_with(".tsx"),
            ..Default::default()
        }),
        &|_metadata| {
            visit_mut_pass(GraphQLVisitor::new(GraphQLCodegenOptions {
                filename: relative_file_path.to_string_lossy().to_string(),
                cwd: cwd.to_string_lossy().to_string(),
                artifact_directory: cwd.to_string_lossy().to_string(),
                gql_tag_name: "gql".to_string(),
                naming_convention: "change-case-all#pascalCase".to_string(),
                document_variable_prefix: "".to_string(),
                document_variable_suffix: "Document".to_string(),
                fragment_variable_prefix: "".to_string(),
                fragment_variable_suffix: "FragmentDoc".to_string(),
                dedupe_operation_suffix: false,
                omit_operation_suffix: false,
            }))
        },
        &input_path,
        &output_path,
        Default::default(),
    );
}

fn get_windows_path_visitor() -> GraphQLVisitor {
    // Simulate a Windows environment where cwd and artifact_directory use
    // backslashes. The WASM plugin runs with Unix path semantics, so it must
    // normalize these.
    GraphQLVisitor::new(GraphQLCodegenOptions {
        filename: "C:\\Users\\user\\project\\src\\App.tsx".to_string(),
        cwd: "C:\\Users\\user\\project".to_string(),
        artifact_directory: "C:\\Users\\user\\project\\src\\gql".to_string(),
        gql_tag_name: "gql".to_string(),
        naming_convention: "change-case-all#pascalCase".to_string(),
        document_variable_prefix: "".to_string(),
        document_variable_suffix: "Document".to_string(),
        fragment_variable_prefix: "".to_string(),
        fragment_variable_suffix: "FragmentDoc".to_string(),
        dedupe_operation_suffix: false,
        omit_operation_suffix: false,
    })
}

test!(
    Default::default(),
    |_| visit_mut_pass(get_windows_path_visitor()),
    windows_absolute_paths_produce_valid_relative_import,
    r#"import gql from "gql-tag";

const GetUser = gql(`
  query GetUser {
    user {
      id
    }
  }
`);"#
);

#[test]
fn windows_absolute_filename_path_gets_correct_relative_import_path() {
    let visitor = get_windows_path_visitor();
    let import_path = visitor.get_relative_import_path("graphql");

    assert_eq!(import_path, "./gql/graphql");
}

test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor()),
    expect_normal_declarations_to_not_panic_and_to_be_ignored,
    // Example from Next.js' server.js
    r#"const emitter = (0, _mitt).default();
    const looseToArray = (input)=>[].slice.call(input);
    const targetTag = document.querySelector(`style[data-n-href="${href}"]`);"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor()),
    preserves_use_cache_directive,
    r#""use cache";

import gql from "gql-tag";

const GetUser = gql(`
  query GetUser {
    user {
      id
      name
    }
  }
`);

const CreatePost = gql(`
  mutation CreatePost($input: PostInput!) {
    createPost(input: $input) {
      id
      title
    }
  }
`);"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor()),
    preserves_multiple_directives,
    r#""use strict";
"use cache";

import gql from "gql-tag";

const GetData = gql(`
  query GetData {
    data
  }
`);"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor()),
    works_without_directives,
    r#"import gql from "gql-tag";

const GetData = gql(`
  query GetData {
    data
  }
`);"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor()),
    pascal_case_converts_eg_word_boundaries,
    r#"import gql from "gql-tag";

const SomeEGRockets = gql(`
  query SomeEGRockets {
    rockets
  }
`);"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor()),
    pascal_case_preserves_digit_suffix_boundaries,
    r#"import gql from "gql-tag";

const Hero30 = gql(`
  fragment Hero30 on SomeType {
    id
  }
`);

const Foo1 = gql(`
  query Foo1 {
    id
  }
`);"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor_upper_case_first()),
    upper_case_first_preserves_uppercase_sequences,
    r#"import gql from "gql-tag";

const SomeEGRockets = gql(`
  query SomeEGRockets {
    rockets
  }
`);"#
);

fn get_test_code_visitor_empty_document_suffix() -> GraphQLVisitor {
    GraphQLVisitor::new(GraphQLCodegenOptions {
        filename: "test.ts".to_string(),
        cwd: "/home/faketestproject".to_string(),
        artifact_directory: "./src/gql".to_string(),
        gql_tag_name: "gql".to_string(),
        naming_convention: "change-case-all#pascalCase".to_string(),
        document_variable_prefix: "".to_string(),
        document_variable_suffix: "".to_string(),
        fragment_variable_prefix: "".to_string(),
        fragment_variable_suffix: "FragmentDoc".to_string(),
        dedupe_operation_suffix: false,
        omit_operation_suffix: false,
    })
}

fn get_test_code_visitor_custom_suffixes() -> GraphQLVisitor {
    GraphQLVisitor::new(GraphQLCodegenOptions {
        filename: "test.ts".to_string(),
        cwd: "/home/faketestproject".to_string(),
        artifact_directory: "./src/gql".to_string(),
        gql_tag_name: "gql".to_string(),
        naming_convention: "change-case-all#pascalCase".to_string(),
        document_variable_prefix: "".to_string(),
        document_variable_suffix: "Doc".to_string(),
        fragment_variable_prefix: "".to_string(),
        fragment_variable_suffix: "Frag".to_string(),
        dedupe_operation_suffix: false,
        omit_operation_suffix: false,
    })
}

test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor_empty_document_suffix()),
    empty_document_suffix_uses_operation_name_directly,
    r#"import gql from "gql-tag";

const GetUser = gql(`
  query GetUser {
    user {
      id
    }
  }
`);"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor_custom_suffixes()),
    custom_document_suffix,
    r#"import gql from "gql-tag";

const GetUser = gql(`
  query GetUser {
    user {
      id
    }
  }
`);"#
);

test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor_custom_suffixes()),
    custom_fragment_suffix,
    r#"import gql from "gql-tag";

const UserFields = gql(`
  fragment UserFields on User {
    id
    name
  }
`);"#
);

fn get_test_code_visitor_with_prefix() -> GraphQLVisitor {
    GraphQLVisitor::new(GraphQLCodegenOptions {
        filename: "test.ts".to_string(),
        cwd: "/home/faketestproject".to_string(),
        artifact_directory: "./src/gql".to_string(),
        gql_tag_name: "gql".to_string(),
        naming_convention: "change-case-all#pascalCase".to_string(),
        document_variable_prefix: "Gql_".to_string(),
        document_variable_suffix: "".to_string(),
        fragment_variable_prefix: "Gql_".to_string(),
        fragment_variable_suffix: "".to_string(),
        dedupe_operation_suffix: false,
        omit_operation_suffix: false,
    })
}

test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor_with_prefix()),
    document_variable_prefix_applied,
    r#"import gql from "gql-tag";

const GetUser = gql(`
  query GetUser {
    user {
      id
    }
  }
`);"#
);

#[test]
fn plugin_options_default_prefix_and_suffix_values() {
    let options: PluginOptions = serde_json::from_str(
        r#"{"artifactDirectory":"./src/gql"}"#,
    )
    .unwrap();

    assert_eq!(options.documentVariablePrefix, "");
    assert_eq!(options.documentVariableSuffix, "Document");
    assert_eq!(options.fragmentVariablePrefix, "");
    assert_eq!(options.fragmentVariableSuffix, "FragmentDoc");
}

#[test]
fn plugin_options_accept_custom_prefix_and_suffix_values() {
    let options: PluginOptions = serde_json::from_str(
        r#"{
            "artifactDirectory":"./src/gql",
            "documentVariablePrefix":"Gql_",
            "documentVariableSuffix":"",
            "fragmentVariablePrefix":"Gql_",
            "fragmentVariableSuffix":"Frag"
        }"#,
    )
    .unwrap();

    assert_eq!(options.documentVariablePrefix, "Gql_");
    assert_eq!(options.documentVariableSuffix, "");
    assert_eq!(options.fragmentVariablePrefix, "Gql_");
    assert_eq!(options.fragmentVariableSuffix, "Frag");
}

// -- dedupeOperationSuffix / omitOperationSuffix tests --
// These only affect fragment variable names, matching codegen's
// getFragmentVariableName behavior.

fn get_test_code_visitor_dedupe_suffix() -> GraphQLVisitor {
    GraphQLVisitor::new(GraphQLCodegenOptions {
        filename: "test.ts".to_string(),
        cwd: "/home/faketestproject".to_string(),
        artifact_directory: "./src/gql".to_string(),
        gql_tag_name: "gql".to_string(),
        naming_convention: "change-case-all#pascalCase".to_string(),
        document_variable_prefix: "".to_string(),
        document_variable_suffix: "Document".to_string(),
        fragment_variable_prefix: "".to_string(),
        fragment_variable_suffix: "FragmentDoc".to_string(),
        dedupe_operation_suffix: true,
        omit_operation_suffix: false,
    })
}

fn get_test_code_visitor_omit_suffix() -> GraphQLVisitor {
    GraphQLVisitor::new(GraphQLCodegenOptions {
        filename: "test.ts".to_string(),
        cwd: "/home/faketestproject".to_string(),
        artifact_directory: "./src/gql".to_string(),
        gql_tag_name: "gql".to_string(),
        naming_convention: "change-case-all#pascalCase".to_string(),
        document_variable_prefix: "".to_string(),
        document_variable_suffix: "Document".to_string(),
        fragment_variable_prefix: "".to_string(),
        fragment_variable_suffix: "FragmentDoc".to_string(),
        dedupe_operation_suffix: false,
        omit_operation_suffix: true,
    })
}

// Fragment name ends with "Fragment" and suffix starts with "Fragment" →
// dedupe strips overlapping "Fragment" from the suffix, leaving just "Doc".
// So MyFragment → MyFragmentDoc (not MyFragmentFragmentDoc).
test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor_dedupe_suffix()),
    dedupe_strips_overlapping_fragment_suffix,
    r#"import gql from "gql-tag";

const MyFragment = gql(`
  fragment MyFragment on User {
    id
    name
  }
`);"#
);

// Fragment name does NOT end with "Fragment" → dedupe has no effect,
// suffix is appended normally.
test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor_dedupe_suffix()),
    dedupe_no_effect_when_name_does_not_end_with_fragment,
    r#"import gql from "gql-tag";

const UserFields = gql(`
  fragment UserFields on User {
    id
    name
  }
`);"#
);

// dedupeOperationSuffix should NOT affect operation document variables
test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor_dedupe_suffix()),
    dedupe_does_not_affect_operation_document_variable,
    r#"import gql from "gql-tag";

const GetUserQuery = gql(`
  query GetUserQuery {
    user {
      id
    }
  }
`);"#
);

// omitOperationSuffix → fragment suffix becomes empty
test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor_omit_suffix()),
    omit_suffix_removes_fragment_suffix,
    r#"import gql from "gql-tag";

const MyFragment = gql(`
  fragment MyFragment on User {
    id
    name
  }
`);"#
);

// omitOperationSuffix should NOT affect operation document variables
test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor_omit_suffix()),
    omit_suffix_does_not_affect_operation_document_variable,
    r#"import gql from "gql-tag";

const GetUser = gql(`
  query GetUser {
    user {
      id
    }
  }
`);"#
);

// When both flags are set, omit takes precedence (suffix is empty)
fn get_test_code_visitor_omit_and_dedupe() -> GraphQLVisitor {
    GraphQLVisitor::new(GraphQLCodegenOptions {
        filename: "test.ts".to_string(),
        cwd: "/home/faketestproject".to_string(),
        artifact_directory: "./src/gql".to_string(),
        gql_tag_name: "gql".to_string(),
        naming_convention: "change-case-all#pascalCase".to_string(),
        document_variable_prefix: "".to_string(),
        document_variable_suffix: "Document".to_string(),
        fragment_variable_prefix: "".to_string(),
        fragment_variable_suffix: "FragmentDoc".to_string(),
        dedupe_operation_suffix: true,
        omit_operation_suffix: true,
    })
}

test!(
    Default::default(),
    |_| visit_mut_pass(get_test_code_visitor_omit_and_dedupe()),
    omit_takes_precedence_over_dedupe,
    r#"import gql from "gql-tag";

const MyFragment = gql(`
  fragment MyFragment on User {
    id
    name
  }
`);"#
);

#[test]
fn plugin_options_dedupe_and_omit_default_to_false() {
    let options: PluginOptions = serde_json::from_str(
        r#"{"artifactDirectory":"./src/gql"}"#,
    )
    .unwrap();

    assert!(!options.dedupeOperationSuffix);
    assert!(!options.omitOperationSuffix);
}

#[test]
fn plugin_options_accept_dedupe_and_omit_values() {
    let options: PluginOptions = serde_json::from_str(
        r#"{
            "artifactDirectory":"./src/gql",
            "dedupeOperationSuffix": true,
            "omitOperationSuffix": true
        }"#,
    )
    .unwrap();

    assert!(options.dedupeOperationSuffix);
    assert!(options.omitOperationSuffix);
}

#[test]
fn naming_convention_keep_preserves_original_name() {
    assert_eq!(
        apply_naming_convention("SomeEGRocketsDocument", "keep"),
        "SomeEGRocketsDocument"
    );
}

#[test]
fn naming_convention_unknown_preserves_original_name() {
    assert_eq!(
        apply_naming_convention("SomeEGRocketsDocument", "lodash#camelCase"),
        "SomeEGRocketsDocument"
    );
}

#[test]
fn pascal_case_preserves_digit_suffixes() {
    assert_eq!(
        apply_naming_convention("Hero30FragmentDoc", "change-case-all#pascalCase"),
        "Hero30FragmentDoc"
    );
    assert_eq!(
        apply_naming_convention("Foo1Document", "change-case-all#pascalCase"),
        "Foo1Document"
    );
    assert_eq!(
        apply_naming_convention("Foo1barDocument", "change-case-all#pascalCase"),
        "Foo1barDocument"
    );
    assert_eq!(
        apply_naming_convention("SomeEGRocketsDocument", "change-case-all#pascalCase"),
        "SomeEgRocketsDocument"
    );
}

#[test]
fn plugin_options_accept_object_naming_convention() {
    let options: PluginOptions = serde_json::from_str(
        r#"{
            "artifactDirectory":"./src/gql",
            "namingConvention":{
                "typeNames":"keep",
                "enumValues":"change-case-all#upperCaseFirst",
                "transformUnderscore":true
            }
        }"#,
    )
    .unwrap();

    assert_eq!(options.namingConvention.as_type_name_convention(), "keep");
}

#[test]
fn plugin_options_object_without_type_names_uses_default_naming_convention() {
    let options: PluginOptions = serde_json::from_str(
        r#"{
            "artifactDirectory":"./src/gql",
            "namingConvention":{
                "enumValues":"keep"
            }
        }"#,
    )
    .unwrap();

    assert_eq!(
        options.namingConvention.as_type_name_convention(),
        "change-case-all#pascalCase"
    );
}

use std::path::PathBuf;

use swc_core::{
    ecma::{
        parser::{Syntax, TsSyntax},
        transforms::testing::{test, test_fixture, test_inline},
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
        document_naming: DocumentNamingOptions::default(),
    })
}

fn get_test_code_visitor_upper_case_first() -> GraphQLVisitor {
    GraphQLVisitor::new(GraphQLCodegenOptions {
        filename: "test.ts".to_string(),
        cwd: "/home/faketestproject".to_string(),
        artifact_directory: "./src/gql".to_string(),
        gql_tag_name: "gql".to_string(),
        naming_convention: "change-case-all#upperCaseFirst".to_string(),
        document_naming: DocumentNamingOptions::default(),
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
                document_naming: DocumentNamingOptions::default(),
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
                document_naming: DocumentNamingOptions::default(),
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
        document_naming: DocumentNamingOptions::default(),
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

fn visitor_with_options(config: &str) -> GraphQLVisitor {
    let options: PluginOptions = serde_json::from_str(config).unwrap();
    let mut visitor = get_test_code_visitor();
    visitor.options.document_naming = options.document_naming;
    visitor.options.naming_convention = options.namingConvention.as_type_name_convention();
    visitor
}

test_inline!(
    Default::default(),
    |_| visit_mut_pass(visitor_with_options(
        r#"{
        "artifactDirectory": "./src/gql",
        "documentVariableSuffix": "",
        "fragmentVariableSuffix": ""
    }"#
    )),
    empty_document_suffixes,
    r#"import gql from "gql-tag";
    const query = gql(`query GetUser { id }`);
    const mutation = gql(`mutation CreateUser { id }`);
    const subscription = gql(`subscription WatchUser { id }`);
    const fragment = gql(`fragment User on User { id }`);"#,
    r#"import { GetUser } from "./src/gql/graphql";
    import { CreateUser } from "./src/gql/graphql";
    import { WatchUser } from "./src/gql/graphql";
    import { User } from "./src/gql/graphql";
    import gql from "gql-tag";
    const query = GetUser;
    const mutation = CreateUser;
    const subscription = WatchUser;
    const fragment = User;"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(visitor_with_options(
        r#"{
        "artifactDirectory": "./src/gql",
        "documentVariablePrefix": "generated",
        "documentVariableSuffix": "Node",
        "fragmentVariablePrefix": "shared",
        "fragmentVariableSuffix": "Part"
    }"#
    )),
    custom_document_affixes_follow_naming_convention,
    r#"import gql from "gql-tag";
    const query = gql(`query GetUser { id }`);
    const mutation = gql(`mutation CreateUser { id }`);
    const subscription = gql(`subscription WatchUser { id }`);
    const fragment = gql(`fragment User on User { id }`);"#,
    r#"import { GeneratedGetUserNode } from "./src/gql/graphql";
    import { GeneratedCreateUserNode } from "./src/gql/graphql";
    import { GeneratedWatchUserNode } from "./src/gql/graphql";
    import { SharedUserPart } from "./src/gql/graphql";
    import gql from "gql-tag";
    const query = GeneratedGetUserNode;
    const mutation = GeneratedCreateUserNode;
    const subscription = GeneratedWatchUserNode;
    const fragment = SharedUserPart;"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(visitor_with_options(
        r#"{
        "artifactDirectory": "./src/gql",
        "dedupeOperationSuffix": true,
        "namingConvention": "keep"
    }"#
    )),
    dedupe_fragment_suffix_case_insensitively,
    r#"import gql from "gql-tag";
    const query = gql(`query GetUserDocument { id }`);
    const fragment = gql(`fragment UserFRAGMENT on User { id }`);
    const other = gql(`fragment User on User { id }`);"#,
    r#"import { GetUserDocumentDocument } from "./src/gql/graphql";
    import { UserFRAGMENTDoc } from "./src/gql/graphql";
    import { UserFragmentDoc } from "./src/gql/graphql";
    import gql from "gql-tag";
    const query = GetUserDocumentDocument;
    const fragment = UserFRAGMENTDoc;
    const other = UserFragmentDoc;"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(visitor_with_options(
        r#"{
        "artifactDirectory": "./src/gql",
        "omitOperationSuffix": true,
        "dedupeOperationSuffix": true,
        "documentVariablePrefix": "op",
        "documentVariableSuffix": "Node",
        "fragmentVariablePrefix": "shared",
        "fragmentVariableSuffix": "FragmentNode"
    }"#
    )),
    omit_fragment_suffix_takes_precedence,
    r#"import gql from "gql-tag";
    const query = gql(`query GetUser { id }`);
    const fragment = gql(`fragment UserFragment on User { id }`);"#,
    r#"import { OpGetUserNode } from "./src/gql/graphql";
    import { SharedUserFragment } from "./src/gql/graphql";
    import gql from "gql-tag";
    const query = OpGetUserNode;
    const fragment = SharedUserFragment;"#
);

#[test]
fn document_naming_defaults_and_custom_fragment_deduplication() {
    let options: PluginOptions =
        serde_json::from_str(r#"{"artifactDirectory":"./src/gql"}"#).unwrap();
    let mut naming = options.document_naming;
    assert_eq!(naming.operation_name("GetUser"), "GetUserDocument");
    assert_eq!(
        naming.fragment_name("UserFragment"),
        "UserFragmentFragmentDoc"
    );
    assert!(!naming.dedupe_operation_suffix);
    assert!(!naming.omit_operation_suffix);

    naming.dedupe_operation_suffix = true;
    naming.fragment_variable_prefix = "Shared".to_string();
    naming.fragment_variable_suffix = "fRaGmEnTNode".to_string();
    assert_eq!(
        naming.fragment_name("UserFragment"),
        "SharedUserFragmentNode"
    );
    naming.fragment_variable_suffix = "Node".to_string();
    assert_eq!(
        naming.fragment_name("UserFragment"),
        "SharedUserFragmentNode"
    );
    naming.fragment_variable_suffix = "Fragment".to_string();
    assert_eq!(naming.fragment_name("UserFragment"), "SharedUserFragment");
}

test_inline!(
    Default::default(),
    |_| visit_mut_pass(visitor_with_options(
        r#"{
        "artifactDirectory": "./src/gql",
        "documentVariablePrefix": "gql_",
        "documentVariableSuffix": "_Document",
        "fragmentVariablePrefix": "shared_",
        "fragmentVariableSuffix": "_FragmentDoc"
    }"#
    )),
    custom_document_affixes_preserve_underscores,
    r#"import gql from "gql-tag";
    const query = gql(`query GetUser { id }`);
    const mutation = gql(`mutation CreateUser { id }`);
    const subscription = gql(`subscription WatchUser { id }`);
    const fragment = gql(`fragment User on User { id }`);"#,
    r#"import { Gql_GetUser_Document } from "./src/gql/graphql";
    import { Gql_CreateUser_Document } from "./src/gql/graphql";
    import { Gql_WatchUser_Document } from "./src/gql/graphql";
    import { Shared_User_FragmentDoc } from "./src/gql/graphql";
    import gql from "gql-tag";
    const query = Gql_GetUser_Document;
    const mutation = Gql_CreateUser_Document;
    const subscription = Gql_WatchUser_Document;
    const fragment = Shared_User_FragmentDoc;"#
);

#[test]
fn naming_conventions_preserve_underscore_boundaries() {
    for (convention, expected) in [
        (
            "change-case-all#pascalCase",
            "_Gql__SomeEgRockets_Document_",
        ),
        (
            "change-case-all#upperCaseFirst",
            "_Gql__SomeEGRockets_Document_",
        ),
        ("keep", "_gql__someEGRockets_Document_"),
    ] {
        assert_eq!(
            apply_naming_convention("_gql__someEGRockets_Document_", convention),
            expected
        );
    }
}

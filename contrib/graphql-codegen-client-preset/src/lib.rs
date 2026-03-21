use std::path::PathBuf;

use graphql_parser::query::parse_query;
use pathdiff::diff_paths;
use serde::Deserialize;
use swc_core::{
    common::Span,
    ecma::{
        ast::*,
        utils::quote_ident,
        visit::{visit_mut_pass, VisitMut, VisitMutWith},
    },
    plugin::{
        errors::HANDLER, metadata::TransformPluginMetadataContextKind, plugin_transform,
        proxies::TransformPluginProgramMetadata,
    },
};

fn upper_case_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn to_pascal_case(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut words: Vec<String> = Vec::new();
    let mut word_start = 0;

    for i in 1..chars.len() {
        let is_boundary = if !chars[i - 1].is_alphanumeric() {
            true
        } else if chars[i - 1].is_lowercase() && chars[i].is_uppercase() {
            // lowercase → uppercase (e.g., someEG: e→E)
            true
        } else if i + 1 < chars.len()
            && chars[i - 1].is_uppercase()
            && chars[i].is_uppercase()
            && chars[i + 1].is_lowercase()
        {
            // uppercase sequence followed by lowercase (e.g., EGRockets: G→R)
            true
        } else {
            false
        };

        if is_boundary {
            let word: String = chars[word_start..i]
                .iter()
                .filter(|c| c.is_alphanumeric())
                .collect();
            if !word.is_empty() {
                words.push(word);
            }
            word_start = i;
        }
    }

    let word: String = chars[word_start..]
        .iter()
        .filter(|c| c.is_alphanumeric())
        .collect();
    if !word.is_empty() {
        words.push(word);
    }

    words
        .iter()
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let rest = chars.as_str().to_lowercase();
                    format!("{}{}", first.to_uppercase(), rest)
                }
            }
        })
        .collect()
}

fn apply_naming_convention(s: &str, naming_convention: &str) -> String {
    match naming_convention {
        "change-case-all#upperCaseFirst" => upper_case_first(s),
        _ => to_pascal_case(s),
    }
}

/// Returns true if `path` is an absolute path on either Unix (`/foo`) or
/// Windows (`C:/foo`, `C:\foo`).  We cannot rely on `Path::is_absolute()`
/// because when the plugin runs inside a WASM runtime it always uses Unix
/// path semantics, so Windows absolute paths would be misclassified as
/// relative.
fn is_absolute_path(path: &str) -> bool {
    // Unix absolute path
    if path.starts_with('/') {
        return true;
    }
    // Windows absolute path: starts with a drive letter followed by ':' and a
    // slash/backslash, e.g. "C:/..." or "C:\..."
    let mut chars = path.chars();
    if let (Some(drive), Some(':'), Some(sep)) = (chars.next(), chars.next(), chars.next()) {
        if drive.is_ascii_alphabetic() && (sep == '/' || sep == '\\') {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests;

pub struct GraphQLCodegenOptions {
    pub filename: String,
    pub cwd: String,
    pub artifact_directory: String,
    pub gql_tag_name: String,
    pub naming_convention: String,
}

pub struct GraphQLVisitor {
    options: GraphQLCodegenOptions,
    graphql_operations_or_fragments_to_import: Vec<String>,
}

impl GraphQLVisitor {
    pub fn new(options: GraphQLCodegenOptions) -> Self {
        GraphQLVisitor {
            options,
            graphql_operations_or_fragments_to_import: Vec::new(),
        }
    }

    fn handle_error(&self, details: &str, span: Span) {
        let message = format!("@graphql-codegen/client-preset-swc-plugin details: {details}");
        HANDLER.with(|handler| handler.struct_span_err(span, &message).emit());
    }

    fn get_relative_import_path(&self, path_end: &str) -> String {
        // Normalize Windows-style backslashes to forward slashes so that PathBuf
        // works correctly when the plugin runs in a WASM context (which always uses
        // Unix path semantics) but receives Windows paths from the host.
        let cwd = self.options.cwd.replace('\\', "/");
        let filename = self.options.filename.replace('\\', "/");
        let artifact_directory = self.options.artifact_directory.replace('\\', "/");

        // using PathBuf to add the relative path to the artifact directory
        let mut file_full_path = PathBuf::from(&cwd);
        file_full_path.push(&filename);
        let file_s_dirname = file_full_path.parent().unwrap();

        // The resolved artifact directory as seen from the current running SWC plugin
        // working directory.
        // We check for absolute paths ourselves instead of relying on
        // Path::is_absolute(), because on WASM (Unix semantics) a Windows absolute
        // path such as "C:/project/src" would be considered relative.
        let resolved_artifact_directory = if is_absolute_path(&artifact_directory) {
            artifact_directory.to_string()
        } else {
            let mut cwd_path = PathBuf::from(&cwd);
            cwd_path.push(&artifact_directory);
            cwd_path.to_string_lossy().to_string()
        };

        let mut relative = diff_paths(resolved_artifact_directory, file_s_dirname).unwrap();

        let start_of_path = "./";

        // e.g. add 'graphql' to relative path
        relative.push(path_end);

        let platform_specific_path = start_of_path.to_string() + relative.to_str().unwrap();
        platform_specific_path.replace('\\', "/")
    }
}

pub fn create_graphql_codegen_visitor(options: GraphQLCodegenOptions) -> impl VisitMut {
    GraphQLVisitor::new(options)
}

impl VisitMut for GraphQLVisitor {
    fn visit_mut_var_decl(&mut self, e: &mut VarDecl) {
        e.visit_mut_children_with(self);

        for decl in e.decls.iter_mut() {
            if let Some(init) = &mut decl.init {
                if let Expr::Call(CallExpr { callee, args, .. }) = &mut **init {
                    if args.is_empty() {
                        return;
                    }

                    match callee.as_expr() {
                        Some(expr_box) => match &**expr_box {
                            Expr::Ident(ident) => {
                                if ident.sym != *self.options.gql_tag_name {
                                    return;
                                }
                            }
                            _ => return,
                        },
                        _ => return,
                    }

                    let quasis = match &*args[0].expr {
                        Expr::Tpl(tpl) => &tpl.quasis,
                        _ => return,
                    };

                    let raw = match &quasis[0].cooked {
                        Some(cooked) => cooked,
                        None => return,
                    };

                    let Some(raw) = raw.as_str() else {
                        return;
                    };

                    let graphql_ast = match parse_query::<&str>(raw) {
                        Ok(ast) => ast,
                        Err(e) => {
                            // Currently the parser outputs a string like: "query parse error", so
                            // we add "GraphQL" to the beginning
                            let error = format!("GraphQL {e}");
                            self.handle_error(error.as_str(), quasis[0].span);
                            return;
                        }
                    };

                    let first_definition = match graphql_ast.definitions.first() {
                        Some(definition) => definition,
                        None => return,
                    };

                    let operation_name = match first_definition {
                        graphql_parser::query::Definition::Fragment(fragment) => {
                            fragment.name.to_string() + "FragmentDoc"
                        }
                        graphql_parser::query::Definition::Operation(op) => match op {
                            graphql_parser::query::OperationDefinition::Query(query) => {
                                match query.name {
                                    Some(name) => name.to_string() + "Document",
                                    None => return,
                                }
                            }
                            graphql_parser::query::OperationDefinition::Mutation(mutation) => {
                                match mutation.name {
                                    Some(name) => name.to_string() + "Document",
                                    None => return,
                                }
                            }
                            graphql_parser::query::OperationDefinition::Subscription(
                                subscription,
                            ) => match subscription.name {
                                Some(name) => name.to_string() + "Document",
                                None => return,
                            },
                            _ => return,
                        },
                    };

                    let import_name = apply_naming_convention(
                        &operation_name,
                        &self.options.naming_convention,
                    );

                    self.graphql_operations_or_fragments_to_import
                        .push(import_name.clone());

                    // now change the call expression to a Identifier
                    let new_expr = Expr::Ident(quote_ident!(import_name).into());

                    *init = Box::new(new_expr);
                }
            }
        }
    }

    fn visit_mut_module(&mut self, module: &mut Module) {
        // First visit all its children, collect the GraphQL document names, and then
        // add the necessary imports
        module.visit_mut_children_with(self);

        if self.graphql_operations_or_fragments_to_import.is_empty() {
            return;
        }

        let platform_specific_path = self.get_relative_import_path("graphql");

        // Find the position after any directive prologue (e.g., "use strict", "use
        // cache")
        let mut insert_position = 0;
        for (index, item) in module.body.iter().enumerate() {
            match item {
                ModuleItem::Stmt(Stmt::Expr(ExprStmt { expr, .. })) => {
                    if let Expr::Lit(Lit::Str(str_lit)) = &**expr {
                        // Check if this is a directive (string literal at the start of a module)
                        if str_lit.value.starts_with("use ") {
                            insert_position = index + 1;
                            continue;
                        }
                    }
                    break;
                }
                _ => break,
            }
        }

        for (i, operation_or_fragment_name) in self
            .graphql_operations_or_fragments_to_import
            .iter()
            .enumerate()
        {
            module.body.insert(
                insert_position + i,
                ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                    span: Default::default(),
                    specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
                        span: Default::default(),
                        local: quote_ident!(operation_or_fragment_name.to_string()).into(),
                        imported: None,
                        is_type_only: false,
                    })],
                    src: Box::new(Str::from(platform_specific_path.to_string())),
                    type_only: false,
                    with: None,
                    phase: Default::default(),
                })),
            )
        }
    }
}

fn gql_default() -> String {
    "gql".to_string()
}

fn naming_convention_default() -> String {
    "change-case-all#pascalCase".to_string()
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct PluginOptions {
    artifactDirectory: String,

    #[serde(default = "gql_default")]
    gqlTagName: String,

    #[serde(default = "naming_convention_default")]
    namingConvention: String,
}

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let filename = metadata
        .get_context(&TransformPluginMetadataContextKind::Filename)
        .unwrap_or_default();
    let cwd = metadata
        .get_context(&TransformPluginMetadataContextKind::Cwd)
        .unwrap_or_default();

    let plugin_config: PluginOptions =
        serde_json::from_str(&metadata.get_transform_plugin_config().expect(
            "Failed to get plugin config for @swc-contrib/plugin-graphql-codegen-client-preset",
        ))
        .expect("Invalid configuration for @swc-contrib/plugin-graphql-codegen-client-preset");

    let artifact_directory = plugin_config.artifactDirectory;
    if artifact_directory.is_empty() {
        panic!(
            "artifactDirectory is not present in the config for \
             @swc-contrib/plugin-graphql-codegen-client-preset"
        );
    }

    let visitor = create_graphql_codegen_visitor(GraphQLCodegenOptions {
        filename,
        cwd,
        artifact_directory,
        gql_tag_name: plugin_config.gqlTagName,
        naming_convention: plugin_config.namingConvention,
    });

    program.apply(&mut visit_mut_pass(visitor))
}

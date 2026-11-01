use std::{collections::HashSet, path::PathBuf};

use graphql_parser::query::parse_query;
use pathdiff::diff_paths;
use serde::Deserialize;
use swc_core::{
    common::Span,
    ecma::{
        ast::*,
        atoms::Atom,
        utils::quote_ident,
        visit::{visit_mut_pass, Visit, VisitMut, VisitMutWith, VisitWith},
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
        } else if chars[i - 1].is_numeric() && chars[i].is_uppercase() {
            // number → uppercase letter (e.g., Hero30Fragment: 0→F)
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
        "change-case-all#pascalCase" => s
            .split('_')
            .map(to_pascal_case)
            .collect::<Vec<_>>()
            .join("_"),
        "change-case-all#upperCaseFirst" => s
            .split('_')
            .map(upper_case_first)
            .collect::<Vec<_>>()
            .join("_"),
        // GraphQL Codegen supports `keep`; preserve casing in that mode.
        "keep" => s.to_string(),
        // Keep unknown/custom conventions unchanged instead of forcing PascalCase.
        _ => s.to_string(),
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

#[derive(Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DocumentNamingOptions {
    pub document_variable_prefix: String,
    pub document_variable_suffix: String,
    pub fragment_variable_prefix: String,
    pub fragment_variable_suffix: String,
    pub dedupe_operation_suffix: bool,
    pub omit_operation_suffix: bool,
}

impl Default for DocumentNamingOptions {
    fn default() -> Self {
        Self {
            document_variable_prefix: String::new(),
            document_variable_suffix: "Document".to_string(),
            fragment_variable_prefix: String::new(),
            fragment_variable_suffix: "FragmentDoc".to_string(),
            dedupe_operation_suffix: false,
            omit_operation_suffix: false,
        }
    }
}

impl DocumentNamingOptions {
    fn operation_name(&self, name: &str) -> String {
        format!(
            "{}{name}{}",
            self.document_variable_prefix, self.document_variable_suffix
        )
    }

    fn fragment_name(&self, name: &str) -> String {
        let suffix = if self.omit_operation_suffix {
            ""
        } else if self.dedupe_operation_suffix
            && name.to_ascii_lowercase().ends_with("fragment")
            && self
                .fragment_variable_suffix
                .to_ascii_lowercase()
                .starts_with("fragment")
        {
            &self.fragment_variable_suffix["Fragment".len()..]
        } else {
            &self.fragment_variable_suffix
        };
        format!("{}{name}{suffix}", self.fragment_variable_prefix)
    }
}

pub struct GraphQLCodegenOptions {
    pub filename: String,
    pub cwd: String,
    pub artifact_directory: String,
    pub gql_tag_name: String,
    pub naming_convention: String,
    pub document_naming: DocumentNamingOptions,
    /// Module specifiers the tag function is imported from. When set, only
    /// calls to a tag imported from one of these modules are rewritten, and
    /// the tag import is removed once no references to it remain.
    pub gql_import_sources: Vec<String>,
}

pub struct GraphQLVisitor {
    options: GraphQLCodegenOptions,
    graphql_operations_or_fragments_to_import: Vec<String>,
    /// Local names bound to the tag function. `None` matches any identifier
    /// named `gql_tag_name`, which is the behavior without
    /// `gql_import_sources`.
    tag_locals: Option<HashSet<Atom>>,
}

impl GraphQLVisitor {
    pub fn new(options: GraphQLCodegenOptions) -> Self {
        GraphQLVisitor {
            options,
            graphql_operations_or_fragments_to_import: Vec::new(),
            tag_locals: None,
        }
    }

    fn is_tag_callee(&self, callee: &Callee) -> bool {
        let Some(expr) = callee.as_expr() else {
            return false;
        };
        let Expr::Ident(ident) = &**expr else {
            return false;
        };
        match &self.tag_locals {
            Some(locals) => locals.contains(&ident.sym),
            None => ident.sym == *self.options.gql_tag_name,
        }
    }

    fn is_import_source(&self, src: &Str) -> bool {
        self.options
            .gql_import_sources
            .iter()
            .any(|source| src.value == *source.as_str())
    }

    /// Local names of the tag function imported from `gql_import_sources`.
    fn collect_tag_locals(&self, module: &Module) -> HashSet<Atom> {
        let mut locals = HashSet::new();
        for item in &module.body {
            let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item else {
                continue;
            };
            if import.type_only || !self.is_import_source(&import.src) {
                continue;
            }
            for specifier in &import.specifiers {
                let ImportSpecifier::Named(named) = specifier else {
                    continue;
                };
                if named.is_type_only {
                    continue;
                }
                let imported = match &named.imported {
                    Some(ModuleExportName::Ident(ident)) => ident.sym.clone(),
                    Some(ModuleExportName::Str(name)) => match name.value.as_str() {
                        Some(value) => Atom::from(value),
                        None => continue,
                    },
                    None => named.local.sym.clone(),
                };
                if imported == *self.options.gql_tag_name {
                    locals.insert(named.local.sym.clone());
                }
            }
        }
        locals
    }

    /// Removes tag import specifiers that no longer have references, and the
    /// import declarations they leave empty.
    fn remove_unused_tag_imports(&self, module: &mut Module, tag_locals: &HashSet<Atom>) {
        let mut finder = ReferenceFinder {
            names: tag_locals,
            found: HashSet::new(),
        };
        module.visit_with(&mut finder);
        let unused: HashSet<&Atom> = tag_locals
            .iter()
            .filter(|local| !finder.found.contains(*local))
            .collect();
        if unused.is_empty() {
            return;
        }

        module.body.retain_mut(|item| {
            let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item else {
                return true;
            };
            if !self.is_import_source(&import.src) || import.specifiers.is_empty() {
                return true;
            }
            import.specifiers.retain(|specifier| match specifier {
                ImportSpecifier::Named(named) => !unused.contains(&named.local.sym),
                _ => true,
            });
            !import.specifiers.is_empty()
        });
    }

    /// The generated document export for a tag call, or `None` when the call
    /// is not a tagged GraphQL document.
    fn document_import_name(&self, call: &CallExpr) -> Option<String> {
        if call.args.is_empty() || !self.is_tag_callee(&call.callee) {
            return None;
        }

        let quasis = match &*call.args[0].expr {
            Expr::Tpl(tpl) => &tpl.quasis,
            _ => return None,
        };

        let raw = quasis[0].cooked.as_ref()?.as_str()?;

        let graphql_ast = match parse_query::<&str>(raw) {
            Ok(ast) => ast,
            Err(e) => {
                // Currently the parser outputs a string like: "query parse error", so
                // we add "GraphQL" to the beginning
                let error = format!("GraphQL {e}");
                self.handle_error(error.as_str(), quasis[0].span);
                return None;
            }
        };

        let operation_name = match graphql_ast.definitions.first()? {
            graphql_parser::query::Definition::Fragment(fragment) => {
                self.options.document_naming.fragment_name(fragment.name)
            }
            graphql_parser::query::Definition::Operation(op) => {
                let name = match op {
                    graphql_parser::query::OperationDefinition::Query(query) => query.name,
                    graphql_parser::query::OperationDefinition::Mutation(mutation) => mutation.name,
                    graphql_parser::query::OperationDefinition::Subscription(subscription) => {
                        subscription.name
                    }
                    _ => None,
                }?;
                self.options.document_naming.operation_name(name)
            }
        };

        Some(apply_naming_convention(
            &operation_name,
            &self.options.naming_convention,
        ))
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

        // Resolve filename first. In WASM runtimes, `PathBuf::push` would treat
        // `C:/...` as relative, so we must detect Windows absolute paths
        // ourselves.
        let file_full_path = if is_absolute_path(&filename) {
            PathBuf::from(&filename)
        } else {
            let mut path = PathBuf::from(&cwd);
            path.push(&filename);
            path
        };
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
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        let Expr::Call(call) = expr else {
            return;
        };
        let Some(import_name) = self.document_import_name(call) else {
            return;
        };

        if !self
            .graphql_operations_or_fragments_to_import
            .contains(&import_name)
        {
            self.graphql_operations_or_fragments_to_import
                .push(import_name.clone());
        }

        *expr = Expr::Ident(quote_ident!(import_name).into());
    }

    fn visit_mut_module(&mut self, module: &mut Module) {
        if !self.options.gql_import_sources.is_empty() {
            let tag_locals = self.collect_tag_locals(module);
            if tag_locals.is_empty() {
                return;
            }
            self.tag_locals = Some(tag_locals);
        }

        // First visit all its children, collect the GraphQL document names, and then
        // add the necessary imports
        module.visit_mut_children_with(self);

        if self.graphql_operations_or_fragments_to_import.is_empty() {
            return;
        }

        if let Some(tag_locals) = self.tag_locals.take() {
            self.remove_unused_tag_imports(module, &tag_locals);
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

/// Records which of `names` are referenced outside import declarations.
struct ReferenceFinder<'a> {
    names: &'a HashSet<Atom>,
    found: HashSet<Atom>,
}

impl Visit for ReferenceFinder<'_> {
    fn visit_import_decl(&mut self, _: &ImportDecl) {}

    fn visit_ident(&mut self, ident: &Ident) {
        if self.names.contains(&ident.sym) {
            self.found.insert(ident.sym.clone());
        }
    }
}

fn gql_default() -> String {
    "gql".to_string()
}

fn naming_convention_default() -> String {
    "change-case-all#pascalCase".to_string()
}

#[derive(Deserialize)]
#[serde(untagged)]
enum NamingConventionOption {
    String(String),
    Object(serde_json::Value),
}

impl NamingConventionOption {
    fn as_type_name_convention(&self) -> String {
        match self {
            NamingConventionOption::String(v) => v.to_string(),
            NamingConventionOption::Object(v) => v
                .get("typeNames")
                .and_then(|t| t.as_str())
                .map(str::to_string)
                .unwrap_or_else(naming_convention_default),
        }
    }
}

fn naming_convention_option_default() -> NamingConventionOption {
    NamingConventionOption::String(naming_convention_default())
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct PluginOptions {
    artifactDirectory: String,

    #[serde(flatten)]
    document_naming: DocumentNamingOptions,

    #[serde(default = "gql_default")]
    gqlTagName: String,

    #[serde(default = "naming_convention_option_default")]
    namingConvention: NamingConventionOption,

    #[serde(default)]
    gqlImportSources: Vec<String>,
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
        naming_convention: plugin_config.namingConvention.as_type_name_convention(),
        document_naming: plugin_config.document_naming,
        gql_import_sources: plugin_config.gqlImportSources,
    });

    program.apply(&mut visit_mut_pass(visitor))
}

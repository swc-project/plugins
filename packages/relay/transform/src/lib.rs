#![allow(clippy::not_unsafe_ptr_arg_deref)]

//! TODO: Once refactoring next-swc is done, remove duplicated codes and import
//! packages directly
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::Arc,
};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use swc_atoms::JsWord;
use swc_common::{FileName, Mark, SyntaxContext, DUMMY_SP};
use swc_ecma_ast::*;
use swc_ecma_utils::{prepend_stmts, quote_ident, ExprFactory};
use swc_ecma_visit::{Fold, FoldWith};

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelayLanguageConfig {
    TypeScript,
    JavaScript,
    Flow,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFileExtension {
    TypeScript,
    JavaScript,
    Undefined,
}

impl<'a> TryFrom<&'a str> for RelayLanguageConfig {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "flow" => Ok(Self::Flow),
            "typescript" => Ok(Self::TypeScript),
            _ => Err(format!("Unexpected language config value '{value}'")),
        }
    }
}

impl<'a> TryFrom<&'a str> for OutputFileExtension {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "ts" => Ok(Self::TypeScript),
            "js" => Ok(Self::JavaScript),
            _ => Err(format!(
                "Unexpected output file extension value '{}'",
                value
            )),
        }
    }
}

impl Default for RelayLanguageConfig {
    fn default() -> Self {
        Self::Flow
    }
}

impl Default for OutputFileExtension {
    fn default() -> Self {
        Self::Undefined
    }
}

#[derive(Debug, Clone)]
struct RelayImport {
    path: JsWord,
    item: JsWord,
    unresolved_mark: Option<Mark>,
}

impl RelayImport {
    fn as_module_item(&self) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
            span: Default::default(),
            specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
                span: Default::default(),
                local: Ident {
                    ctxt: self
                        .unresolved_mark
                        .map(|m| SyntaxContext::empty().apply_mark(m))
                        .unwrap_or_default(),
                    span: DUMMY_SP,
                    sym: self.item.clone(),
                    optional: false,
                },
            })],
            src: Box::new(self.path.clone().into()),
            type_only: false,
            with: None,
            phase: Default::default(),
        }))
    }
}

struct Relay {
    root_dir: PathBuf,
    pages_dir: Option<PathBuf>,
    file_name: FileName,
    config: Arc<Config>,
    imports: Vec<RelayImport>,
    unresolved_mark: Option<Mark>,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub projects: Vec<ProjectConfig>,

    #[serde(default)]
    pub artifact_directory: Option<PathBuf>,

    #[serde(default)]
    pub language: RelayLanguageConfig,

    #[serde(default)]
    pub eager_es_modules: bool,
    #[serde(default)]
    pub output_file_extension: OutputFileExtension,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    pub root_dir: PathBuf,
    #[serde(default)]
    pub artifact_directory: Option<PathBuf>,
}

/// A line starting with `#` is a comment.
fn strip_comments(s: &str) -> Cow<str> {
    if s.contains('#') {
        let mut buf = String::with_capacity(s.len());
        for line in s.lines() {
            if let Some(idx) = line.find('#') {
                buf.push_str(&line[..idx]);
            } else {
                buf.push_str(line);
            }
            buf.push('\n');
        }
        buf.into()
    } else {
        s.into()
    }
}

fn pull_first_operation_name_from_tpl(tpl: &TaggedTpl) -> Option<String> {
    static OPERATION_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(fragment|mutation|query|subscription) (\w+)").unwrap());

    tpl.tpl.quasis.iter().find_map(|quasis| {
        let raw = strip_comments(&quasis.raw);

        let capture_group = OPERATION_REGEX.captures_iter(&raw).next();

        capture_group.map(|capture_group| capture_group[2].to_string())
    })
}

fn build_require_expr_from_path(path: &str, mark: Option<Mark>) -> Expr {
    Expr::Call(CallExpr {
        span: Default::default(),
        callee: quote_ident!(
            mark.map(|m| SyntaxContext::empty().apply_mark(m))
                .unwrap_or_default(),
            "require"
        )
        .as_callee(),
        args: vec![Lit::Str(Str {
            span: Default::default(),
            value: JsWord::from(path),
            raw: None,
        })
        .as_arg()],
        ..Default::default()
    })
}

impl Fold for Relay {
    fn fold_expr(&mut self, expr: Expr) -> Expr {
        let expr = expr.fold_children_with(self);

        match &expr {
            Expr::TaggedTpl(tpl) => {
                if let Some(built_expr) = self.build_call_expr_from_tpl(tpl) {
                    built_expr
                } else {
                    expr
                }
            }
            _ => expr,
        }
    }

    fn fold_module_items(&mut self, items: Vec<ModuleItem>) -> Vec<ModuleItem> {
        let mut items = items
            .into_iter()
            .map(|item| item.fold_children_with(self))
            .collect::<Vec<_>>();

        prepend_stmts(
            &mut items,
            self.imports.iter().map(|import| import.as_module_item()),
        );

        items
    }
}

// TODO: This is really hacky.
fn unique_ident_name_from_operation_name(operation_name: &str) -> String {
    format!("__{}", operation_name)
}

#[derive(Debug)]
enum BuildRequirePathError {
    FileNameNotReal,
    ArtifactDirectoryExpected { file_name: String },
}

impl Relay {
    fn path_for_artifact(
        &self,
        real_file_name: &Path,
        definition_name: &str,
    ) -> Result<PathBuf, BuildRequirePathError> {
        let filename = match &self.config.output_file_extension {
            OutputFileExtension::JavaScript => format!("{}.graphql.js", definition_name),
            OutputFileExtension::TypeScript => format!("{}.graphql.ts", definition_name),
            OutputFileExtension::Undefined => match &self.config.language {
                RelayLanguageConfig::Flow => format!("{}.graphql.js", definition_name),
                RelayLanguageConfig::TypeScript => format!("{}.graphql.ts", definition_name),
                RelayLanguageConfig::JavaScript => format!("{}.graphql.js", definition_name),
            },
        };

        if !self.config.projects.is_empty() {
            for project in &self.config.projects {
                if real_file_name.starts_with(&project.root_dir) {
                    return Ok(project
                        .artifact_directory
                        .as_deref()
                        .unwrap_or_else(|| &self.root_dir)
                        .join("__generated__")
                        .join(filename));
                }
            }
        }

        if let Some(artifact_directory) = &self.config.artifact_directory {
            Ok(self.root_dir.join(artifact_directory).join(filename))
        } else if self
            .pages_dir
            .as_ref()
            .map_or(false, |pages_dir| real_file_name.starts_with(pages_dir))
        {
            Err(BuildRequirePathError::ArtifactDirectoryExpected {
                file_name: real_file_name.display().to_string(),
            })
        } else {
            Ok(real_file_name
                .parent()
                .unwrap()
                .join("./__generated__")
                .join(filename))
        }
    }

    fn build_require_path(
        &mut self,
        operation_name: &str,
    ) -> Result<PathBuf, BuildRequirePathError> {
        match &self.file_name {
            FileName::Real(real_file_name) => {
                self.path_for_artifact(real_file_name, operation_name)
            }
            _ => Err(BuildRequirePathError::FileNameNotReal),
        }
    }

    fn build_call_expr_from_tpl(&mut self, tpl: &TaggedTpl) -> Option<Expr> {
        if let Expr::Ident(ident) = &*tpl.tag {
            if &*ident.sym != "graphql" {
                return None;
            }
        }

        let operation_name = pull_first_operation_name_from_tpl(tpl);

        match operation_name {
            None => None,
            Some(operation_name) => match self.build_require_path(&operation_name) {
                Ok(final_path) => {
                    let final_path = final_path.to_string_lossy();

                    #[cfg(target_os = "windows")]
                    let final_path = final_path.replace("\\", "/");

                    let ident_name: JsWord =
                        unique_ident_name_from_operation_name(&operation_name).into();

                    if self.config.eager_es_modules {
                        self.imports.push(RelayImport {
                            path: final_path.into(),
                            item: ident_name.clone(),
                            unresolved_mark: self.unresolved_mark,
                        });
                        let operation_ident = Ident {
                            ctxt: self
                                .unresolved_mark
                                .map(|m| SyntaxContext::empty().apply_mark(m))
                                .unwrap_or_default(),
                            sym: ident_name,
                            ..Default::default()
                        };
                        Some(Expr::Ident(operation_ident))
                    } else {
                        Some(build_require_expr_from_path(
                            &final_path,
                            self.unresolved_mark,
                        ))
                    }
                }
                Err(_err) => {
                    // let base_error = "Could not transform GraphQL template to a Relay import.";
                    // let error_message = match err {
                    //     BuildRequirePathError::FileNameNotReal => {
                    //         "Source file was not a real file.".to_string()
                    //     }
                    // };

                    // HANDLER.with(|handler| {
                    //     handler.span_err(
                    //         tpl.span,
                    //         format!("{} {}", base_error, error_message).as_str(),
                    //     );
                    // });

                    None
                }
            },
        }
    }
}

pub fn relay(
    config: Arc<Config>,
    file_name: FileName,
    root_dir: PathBuf,
    pages_dir: Option<PathBuf>,
    unresolved_mark: Option<Mark>,
) -> impl Fold {
    Relay {
        root_dir,
        file_name,
        config,
        pages_dir,
        imports: vec![],
        unresolved_mark,
    }
}

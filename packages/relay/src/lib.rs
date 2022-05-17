//! TODO: Once refactoring next-swc is done, remove duplicated codes and import
//! packages directly
use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use swc_atoms::JsWord;
use swc_common::{errors::HANDLER, FileName};
use swc_ecmascript::{
    ast::*,
    utils::{quote_ident, ExprFactory},
    visit::{Fold, FoldWith},
};
use swc_plugin::{plugin_transform, TransformPluginProgramMetadata};

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelayLanguageConfig {
    TypeScript,
    Flow,
}

impl Default for RelayLanguageConfig {
    fn default() -> Self {
        Self::Flow
    }
}

struct Relay<'a> {
    root_dir: PathBuf,
    pages_dir: PathBuf,
    file_name: FileName,
    config: &'a Config,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub src: PathBuf,
    pub artifact_directory: Option<PathBuf>,
    #[serde(default)]
    pub language: RelayLanguageConfig,
}

fn pull_first_operation_name_from_tpl(tpl: &TaggedTpl) -> Option<String> {
    tpl.tpl.quasis.iter().find_map(|quasis| {
        static OPERATION_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(fragment|mutation|query|subscription) (\w+)").unwrap());

        let capture_group = OPERATION_REGEX.captures_iter(&quasis.raw).next();

        capture_group.map(|capture_group| capture_group[2].to_string())
    })
}

fn build_require_expr_from_path(path: &str) -> Expr {
    Expr::Call(CallExpr {
        span: Default::default(),
        callee: quote_ident!("require").as_callee(),
        args: vec![Lit::Str(Str {
            span: Default::default(),
            value: JsWord::from(path),
            raw: None,
        })
        .as_arg()],
        type_args: None,
    })
}

impl<'a> Fold for Relay<'a> {
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
}

#[derive(Debug)]
enum BuildRequirePathError {
    FileNameNotReal,
    ArtifactDirectoryExpected { file_name: String },
}

impl<'a> Relay<'a> {
    fn path_for_artifact(
        &self,
        real_file_name: &Path,
        definition_name: &str,
    ) -> Result<PathBuf, BuildRequirePathError> {
        let filename = match &self.config.language {
            RelayLanguageConfig::Flow => format!("{}.graphql.js", definition_name),
            RelayLanguageConfig::TypeScript => {
                format!("{}.graphql.ts", definition_name)
            }
        };

        if let Some(artifact_directory) = &self.config.artifact_directory {
            Ok(self.root_dir.join(artifact_directory).join(filename))
        } else if real_file_name.starts_with(&self.pages_dir) {
            Err(BuildRequirePathError::ArtifactDirectoryExpected {
                file_name: real_file_name.display().to_string(),
            })
        } else {
            Ok(real_file_name
                .parent()
                .unwrap()
                .join("__generated__")
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
            Some(operation_name) => match self.build_require_path(operation_name.as_str()) {
                Ok(final_path) => Some(build_require_expr_from_path(final_path.to_str().unwrap())),
                Err(err) => {
                    let base_error = "Could not transform GraphQL template to a Relay import.";
                    let error_message = match err {
                        BuildRequirePathError::FileNameNotReal => "Source file was not a real \
                                                                   file. This is likely a bug and \
                                                                   should be reported to Next.js"
                            .to_string(),
                        BuildRequirePathError::ArtifactDirectoryExpected { file_name } => {
                            format!(
                                "The generated file for `{}` will be created in `pages` \
                                 directory, which will break production builds. Try moving the \
                                 file outside of `pages` or set the `artifactDirectory` in the \
                                 Relay config file.",
                                file_name
                            )
                        }
                    };

                    HANDLER.with(|handler| {
                        handler.span_err(
                            tpl.span,
                            format!("{} {}", base_error, error_message).as_str(),
                        );
                    });

                    None
                }
            },
        }
    }
}

pub fn relay<'a>(
    config: &'a Config,
    file_name: FileName,
    pages_dir: Option<PathBuf>,
    root_dir: PathBuf,
) -> impl Fold + '_ {
    Relay {
        root_dir,
        file_name,
        pages_dir: pages_dir.unwrap_or_else(|| panic!("pages_dir is expected.")),
        config,
    }
}

#[plugin_transform]
fn relay_plugin_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let context: Value = serde_json::from_str(&metadata.transform_context)
        .expect("Should able to deserialize context");
    let filename = if let Some(filename) = (&context["filename"]).as_str() {
        FileName::Real(PathBuf::from(filename))
    } else {
        FileName::Anon
    };

    let plugin_config: Value =
        serde_json::from_str(&metadata.plugin_config).expect("Should provide plugin config");

    let pages_dir = PathBuf::from(
        (&plugin_config["pagesDir"])
            .as_str()
            .expect("pages_dir is expected."),
    );

    let src = PathBuf::from((&plugin_config["src"]).as_str().expect("src is expected"));
    let artifact_directory = (&plugin_config["artifactDirectory"])
        .as_str()
        .map(|v| PathBuf::from(v));
    let language = (&plugin_config["language"]).as_str().map_or(
        RelayLanguageConfig::TypeScript,
        |v| match v {
            "typescript" => RelayLanguageConfig::TypeScript,
            "flow" => RelayLanguageConfig::Flow,
            _ => panic!("Unexpected language config value"),
        },
    );

    let config = Config {
        src,
        artifact_directory,
        language,
    };

    let mut relay = relay(&config, filename, Some(pages_dir), PathBuf::from("/cwd"));
    let program = program.fold_with(&mut relay);

    program
}

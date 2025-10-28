use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    path::Path,
};

use base64ct::{Base64, Base64UrlUnpadded, Encoding};
use digest::DynDigest;
use md5::Md5;
use once_cell::sync::Lazy;
use regex::{Captures, Regex as Regexp};
use serde::{ser::SerializeMap, Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Digest, Sha512};
use swc_core::{
    common::{
        comments::{Comment, CommentKind, Comments},
        source_map::SmallPos,
        BytePos, Loc, SourceMapper, Span, Spanned, DUMMY_SP,
    },
    ecma::{
        ast::{
            ArrayLit, AssignExpr, AssignTarget, BinExpr, BinaryOp, Bool, CallExpr, Callee, Expr,
            ExprOrSpread, Id, IdentName, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue,
            JSXElementName, JSXExpr, JSXExprContainer, JSXNamespacedName, JSXOpeningElement,
            KeyValueProp, Lit, MemberProp, ModuleItem, Number, ObjectLit, Pat, Prop, PropName,
            PropOrSpread, SimpleAssignTarget, Str, Tpl, VarDeclarator,
        },
        visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
    },
};
use swc_icu_messageformat_parser::{Parser, ParserOptions};

pub static WHITESPACE_REGEX: Lazy<Regexp> = Lazy::new(|| Regexp::new(r"\s+").unwrap());

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FormatJSPluginOptions {
    pub pragma: Option<String>,
    pub remove_default_message: bool,
    pub id_interpolation_pattern: Option<String>,
    pub ast: bool,
    pub extract_source_location: bool,
    pub preserve_whitespace: bool,
    pub __debug_extracted_messages_comment: bool,
    pub additional_function_names: Vec<String>,
    pub additional_component_names: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct JSXMessageDescriptorPath {
    id: Option<JSXAttrValue>,
    default_message: Option<JSXAttrValue>,
    description: Option<JSXAttrValue>,
}

#[derive(Debug, Clone, Default)]
pub struct CallExprMessageDescriptorPath {
    id: Option<Expr>,
    default_message: Option<Expr>,
    description: Option<Expr>,
}

#[derive(Debug, Clone, Default)]
pub struct MessageDescriptor {
    id: Option<String>,
    default_message: Option<String>,
    description: Option<MessageDescriptionValue>,
}

fn parse(source: &str) -> Result<Box<Expr>, swc_icu_messageformat_parser::Error> {
    let options = ParserOptions {
        should_parse_skeletons: true,
        requires_other_clause: true,
        ..ParserOptions::default()
    };
    let mut parser = Parser::new(source, &options);
    match parser.parse() {
        Ok(parsed) => {
            let v = serde_json::to_value(&parsed).unwrap();
            Ok(json_value_to_expr(&v))
        }
        Err(e) => Err(e),
    }
}

// TODO: consolidate with get_message_descriptor_key_from_call_expr?
fn get_message_descriptor_key_from_jsx(name: &JSXAttrName) -> &str {
    match name {
        JSXAttrName::Ident(name)
        | JSXAttrName::JSXNamespacedName(JSXNamespacedName { name, .. }) => &name.sym,
    }

    // NOTE: Do not support evaluatePath()
}

fn get_message_descriptor_key_from_call_expr(name: &PropName) -> Option<&str> {
    match name {
        PropName::Ident(name) => Some(&*name.sym),
        PropName::Str(name) => Some(name.value.as_str().expect("non-utf8 prop name")),
        _ => None,
    }

    // NOTE: Do not support evaluatePath()
}

// TODO: Consolidate with create_message_descriptor_from_call_expr
fn create_message_descriptor_from_jsx_attr(
    attrs: &Vec<JSXAttrOrSpread>,
) -> JSXMessageDescriptorPath {
    let mut ret = JSXMessageDescriptorPath::default();
    for attr in attrs {
        if let JSXAttrOrSpread::JSXAttr(JSXAttr { name, value, .. }) = attr {
            let key = get_message_descriptor_key_from_jsx(name);

            match key {
                "id" => {
                    ret.id = value.clone();
                }
                "defaultMessage" => {
                    ret.default_message = value.clone();
                }
                "description" => {
                    ret.description = value.clone();
                }
                _ => {
                    //unexpected
                }
            }
        }
    }

    ret
}

fn create_message_descriptor_from_call_expr(
    props: &Vec<PropOrSpread>,
) -> CallExprMessageDescriptorPath {
    let mut ret = CallExprMessageDescriptorPath::default();
    for prop in props {
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(KeyValueProp { key, value }) = &**prop {
                if let Some(key) = get_message_descriptor_key_from_call_expr(key) {
                    match key {
                        "id" => {
                            ret.id = Some(*value.clone());
                        }
                        "defaultMessage" => {
                            ret.default_message = Some(*value.clone());
                        }
                        "description" => {
                            ret.description = Some(*value.clone());
                        }
                        _ => {
                            //unexpected
                        }
                    }
                };
            }
        }
    }

    ret
}

fn get_jsx_message_descriptor_value(
    value: Option<&JSXAttrValue>,
    is_message_node: Option<bool>,
) -> Option<String> {
    let value = value?;

    // NOTE: do not support evaluatePath
    match value {
        JSXAttrValue::JSXExprContainer(container) => {
            if is_message_node.unwrap_or(false) {
                if let JSXExpr::Expr(expr) = &container.expr {
                    // If this is already compiled, no need to recompiled it
                    if let Expr::Array(..) = &**expr {
                        return None;
                    }
                }
            }

            match &container.expr {
                JSXExpr::Expr(expr) => match &**expr {
                    Expr::Lit(Lit::Str(s)) => {
                        Some(s.value.as_str().expect("non-utf8 string").to_string())
                    }
                    Expr::Tpl(tpl) => Some(evaluate_template_literal_string(tpl)),
                    Expr::Bin(bin_expr) => evaluate_binary_expr(bin_expr),
                    _ => None,
                },
                _ => None,
            }
        }
        JSXAttrValue::Str(s) => Some(s.value.as_str().expect("non-utf8 string").to_string()),
        _ => None,
    }
}

/// Helper function to evaluate binary expressions (string concatenation)
fn evaluate_binary_expr(expr: &BinExpr) -> Option<String> {
    // Only handle string concatenation (+ operator)
    if !matches!(expr.op, BinaryOp::Add) {
        return None;
    }

    // Recursively evaluate left and right operands
    let left_str = get_call_expr_message_descriptor_value(Some(&*expr.left), None)?;
    let right_str = get_call_expr_message_descriptor_value(Some(&*expr.right), None)?;

    Some(format!("{left_str}{right_str}"))
}

fn get_call_expr_message_descriptor_value(
    value: Option<&Expr>,
    _is_message_node: Option<bool>,
) -> Option<String> {
    let value = value?;

    // NOTE: do not support evaluatePath
    match value {
        Expr::Ident(ident) => Some(ident.sym.to_string()),
        Expr::Lit(Lit::Str(s)) => Some(s.value.as_str().expect("non-utf8 string").to_string()),
        Expr::Tpl(tpl) => {
            //NOTE: This doesn't fully evaluate templates
            Some(
                tpl.quasis
                    .iter()
                    .map(|q| {
                        q.cooked
                            .as_ref()
                            .map(|v| v.as_str().expect("non-utf8 string").to_string())
                            .unwrap_or("".to_string())
                    })
                    .collect::<Vec<String>>()
                    .join(""),
            )
        }
        Expr::Bin(bin_expr) => evaluate_binary_expr(bin_expr),
        _ => None,
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum MessageDescriptionValue {
    Str(String),
    Obj(ObjectLit),
}

impl Serialize for MessageDescriptionValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MessageDescriptionValue::Str(s) => serializer.serialize_str(s),
            // NOTE: this is good enough to barely pass key-value object serialization. Not a
            // complete implementation.
            MessageDescriptionValue::Obj(obj) => {
                let mut state = serializer.serialize_map(Some(obj.props.len()))?;
                for prop in &obj.props {
                    match prop {
                        PropOrSpread::Prop(prop) => {
                            match &**prop {
                                Prop::KeyValue(key_value) => {
                                    let key = match &key_value.key {
                                        PropName::Ident(ident) => ident.sym.to_string(),
                                        PropName::Str(s) => s.value.to_atom_lossy().to_string(),
                                        _ => {
                                            //unexpected
                                            continue;
                                        }
                                    };
                                    let value = match &*key_value.value {
                                        Expr::Lit(Lit::Str(s)) => {
                                            s.value.to_atom_lossy().to_string()
                                        }
                                        _ => {
                                            //unexpected
                                            continue;
                                        }
                                    };
                                    state.serialize_entry(&key, &value)?;
                                }
                                _ => {
                                    //unexpected
                                    continue;
                                }
                            }
                        }
                        _ => {
                            //unexpected
                            continue;
                        }
                    }
                }
                state.end()
            }
        }
    }
}

fn get_jsx_icu_message_value(
    message_path: Option<&JSXAttrValue>,
    preserve_whitespace: bool,
) -> String {
    let message = message_path
        .and_then(|path| get_jsx_message_descriptor_value(Some(path), Some(true)))
        .unwrap_or("".to_string());

    let message = if !preserve_whitespace {
        let message = WHITESPACE_REGEX.replace_all(&message, " ");
        message.trim().to_string()
    } else {
        message
    };

    if let Err(e) = parse(message.as_str()) {
        let is_literal_err = if let Some(JSXAttrValue::Str(..)) = message_path {
            message.contains("\\\\")
        } else {
            false
        };

        let handler = &swc_core::plugin::errors::HANDLER;

        if is_literal_err {
            {
                handler.with(|handler| {
                    handler
                        .struct_err(
                            r#"
                    [React Intl] Message failed to parse.
                    It looks like `\\`s were used for escaping,
                    this won't work with JSX string literals.
                    Wrap with `{{}}`.
                    See: http://facebook.github.io/react/docs/jsx-gotchas.html
                    "#,
                        )
                        .emit()
                });
            }
        } else {
            {
                handler.with(|handler| {
                    handler
                        .struct_warn(
                            r#"
                    [React Intl] Message failed to parse.
                    See: https://formatjs.io/docs/core-concepts/icu-syntax
                    \n {:#?}
                    "#,
                        )
                        .emit();
                    handler
                        .struct_err(&format!("SyntaxError: {}", e.kind))
                        .emit()
                });
            }
        }
    }

    message
}

fn get_call_expr_icu_message_value(
    message_path: Option<&Expr>,
    preserve_whitespace: bool,
) -> String {
    let message = message_path
        .and_then(|path| get_call_expr_message_descriptor_value(Some(path), Some(true)))
        .unwrap_or("".to_string());

    let message = if !preserve_whitespace {
        let message = WHITESPACE_REGEX.replace_all(&message, " ");
        message.trim().to_string()
    } else {
        message
    };

    if let Err(e) = parse(message.as_str()) {
        let handler = &swc_core::plugin::errors::HANDLER;

        {
            handler.with(|handler| {
                handler
                    .struct_warn(
                        r#"
                    [React Intl] Message failed to parse.
                    See: https://formatjs.io/docs/core-concepts/icu-syntax
                    \n {:#?}
                    "#,
                    )
                    .emit();
                handler
                    .struct_err(&format!("SyntaxError: {}", e.kind))
                    .emit()
            });
        }
    }

    message
}

fn interpolate_name(filename: &str, interpolate_pattern: &str, content: &str) -> Option<String> {
    let mut resource_path = filename.to_string();
    let mut basename = "file";

    let path = Path::new(filename);
    let parent = path.parent();
    if let Some(parent) = parent {
        let parent_str = parent.to_str().unwrap();
        if !parent_str.is_empty() {
            basename = path.file_stem()?.to_str().unwrap();
            resource_path = format!("{parent_str}/");
        }
    }

    let mut directory: String;
    directory = resource_path.replace("\\", "/").to_owned();
    directory = Regexp::new(r#"\.\.(/)?"#)
        .unwrap()
        .replace(directory.as_str(), "_$1")
        .to_string();

    let folder = match directory.len() {
        0 | 1 => {
            directory = "".to_string();
            ""
        }
        _ => Path::new(&directory)
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or(""),
    };

    let mut url = interpolate_pattern.to_string();
    let r =
        Regexp::new(r#"\[(?:([^:\]]+):)?(?:hash|contenthash)(?::([a-z][a-z0-9]*))?(?::(\d+))?\]"#)
            .unwrap();

    url = r
        .replace(url.as_str(), |cap: &Captures| {
            let hash_type = cap.get(1);
            let digest_encoding_type = cap.get(2);
            let max_length = cap.get(3);

            // TODO: support more hash_types than md5, sha1 and sha512
            let mut hasher: Box<dyn DynDigest> = match hash_type {
                Some(hash_type) if hash_type.as_str() == "md5" => Box::new(Md5::new()),
                Some(hash_type) if hash_type.as_str() == "sha1" => Box::new(Sha1::new()),
                _ => Box::new(Sha512::new()),
            };
            hasher.update(content.as_bytes());
            let hash = hasher.finalize();
            let encoded_hash = match digest_encoding_type.map(|m| m.as_str()) {
                Some("base64") => Base64::encode_string(&hash),
                Some("base64url") => Base64UrlUnpadded::encode_string(&hash),
                Some("hex") | None => hex::encode(&hash),
                Some(other) => {
                    swc_core::plugin::errors::HANDLER.with(|handler| {
                        handler.warn(&format!(
                            "[React Intl] Unsupported encoding type `{other}` in \
                             `idInterpolationPattern`, must be one of `hex`, `base64`, or \
                             `base64url`."
                        ))
                    });

                    hex::encode(&hash)
                }
            };

            if let Some(max_length) = max_length {
                encoded_hash[0..max_length.as_str().parse::<usize>().unwrap()].to_string()
            } else {
                encoded_hash
            }
        })
        .to_string();

    url = Regexp::new(r#"\[(ext|name|path|folder|query)\]"#)
        .unwrap()
        .replace_all(url.as_str(), |cap: &Captures| {
            if let Some(placeholder) = cap.get(1) {
                match placeholder.as_str() {
                    "ext" => {
                        if let Some(extension) = path.extension() {
                            extension.to_str().unwrap()
                        } else {
                            "bin"
                        }
                    }
                    "name" => basename,
                    "path" => directory.as_str(),
                    "folder" => folder,
                    "query" => "",
                    _ => panic!("unreachable"),
                }
            } else {
                ""
            }
        })
        .to_string();

    Some(url)
}

// TODO: Consolidate with evaluate_call_expr_message_descriptor
fn evaluate_jsx_message_descriptor_with_visitor(
    descriptor_path: &JSXMessageDescriptorPath,
    options: &FormatJSPluginOptions,
    filename: &str,
    visitor: &FormatJSVisitor<impl Clone + Comments, impl SourceMapper>,
) -> MessageDescriptor {
    let id = get_jsx_message_descriptor_value(descriptor_path.id.as_ref(), None);
    let default_message = get_jsx_icu_message_value(
        descriptor_path.default_message.as_ref(),
        options.preserve_whitespace,
    );

    let description = visitor.get_jsx_message_descriptor_value_maybe_object_with_resolution(
        descriptor_path.description.as_ref(),
        None,
    );

    // Note: do not support override fn
    let id = if id.is_none() && !default_message.is_empty() {
        let interpolate_pattern =
            if let Some(interpolate_pattern) = &options.id_interpolation_pattern {
                interpolate_pattern.as_str()
            } else {
                "[sha512:contenthash:base64:6]"
            };

        let content = if let Some(MessageDescriptionValue::Str(description)) = &description {
            format!("{default_message}#{description}")
        } else if let Some(MessageDescriptionValue::Obj(obj)) = &description {
            // When description is an object, stringify it for the hash calculation
            let mut map = std::collections::BTreeMap::new();
            // Extract and convert properties in one pass
            for prop in &obj.props {
                if let PropOrSpread::Prop(prop) = prop {
                    if let Prop::KeyValue(key_value) = &**prop {
                        let key_str = match &key_value.key {
                            PropName::Ident(ident) => ident.sym.to_string(),
                            PropName::Str(s) => s.value.to_atom_lossy().to_string(),
                            _ => continue,
                        };

                        let value = match &*key_value.value {
                            Expr::Lit(Lit::Str(s)) => {
                                serde_json::Value::String(s.value.to_atom_lossy().to_string())
                            }
                            Expr::Lit(Lit::Num(n)) => serde_json::Number::from_f64(n.value)
                                .map(serde_json::Value::Number)
                                .unwrap_or(serde_json::Value::Null),
                            Expr::Lit(Lit::Bool(b)) => serde_json::Value::Bool(b.value),
                            _ => continue,
                        };

                        map.insert(key_str, value);
                    }
                }
            }

            // Convert BTreeMap to JSON object with keys already sorted
            let json_obj = map
                .into_iter()
                .collect::<serde_json::Map<String, serde_json::Value>>();
            let obj_value = serde_json::Value::Object(json_obj);
            let desc_json = serde_json::to_string(&obj_value).unwrap_or_default();
            format!("{default_message}#{desc_json}")
        } else {
            default_message.clone()
        };

        interpolate_name(filename, interpolate_pattern, &content)
    } else {
        id
    };

    MessageDescriptor {
        id,
        default_message: Some(default_message),
        description,
    }
}

fn evaluate_call_expr_message_descriptor_with_visitor(
    descriptor_path: &CallExprMessageDescriptorPath,
    options: &FormatJSPluginOptions,
    filename: &str,
    visitor: &FormatJSVisitor<impl Clone + Comments, impl SourceMapper>,
) -> MessageDescriptor {
    let id = get_call_expr_message_descriptor_value(descriptor_path.id.as_ref(), None);
    let default_message = get_call_expr_icu_message_value(
        descriptor_path.default_message.as_ref(),
        options.preserve_whitespace,
    );

    let description = visitor.get_call_expr_message_descriptor_value_maybe_object_with_resolution(
        descriptor_path.description.as_ref(),
        None,
    );

    let id = if id.is_none() && !default_message.is_empty() {
        let interpolate_pattern =
            if let Some(interpolate_pattern) = &options.id_interpolation_pattern {
                interpolate_pattern.as_str()
            } else {
                "[sha512:contenthash:base64:6]"
            };

        let content = if let Some(MessageDescriptionValue::Str(description)) = &description {
            format!("{default_message}#{description}")
        } else if let Some(MessageDescriptionValue::Obj(obj)) = &description {
            // When description is an object, stringify it for the hash calculation
            let mut map = std::collections::BTreeMap::new();
            // Extract and convert properties in one pass
            for prop in &obj.props {
                if let PropOrSpread::Prop(prop) = prop {
                    if let Prop::KeyValue(key_value) = &**prop {
                        let key_str = match &key_value.key {
                            PropName::Ident(ident) => ident.sym.to_string(),
                            PropName::Str(s) => s.value.to_atom_lossy().to_string(),
                            _ => continue,
                        };

                        let value = match &*key_value.value {
                            Expr::Lit(Lit::Str(s)) => {
                                serde_json::Value::String(s.value.to_atom_lossy().to_string())
                            }
                            Expr::Lit(Lit::Num(n)) => serde_json::Number::from_f64(n.value)
                                .map(serde_json::Value::Number)
                                .unwrap_or(serde_json::Value::Null),
                            Expr::Lit(Lit::Bool(b)) => serde_json::Value::Bool(b.value),
                            _ => continue,
                        };

                        map.insert(key_str, value);
                    }
                }
            }

            // Convert BTreeMap to JSON object with keys already sorted
            let json_obj = map
                .into_iter()
                .collect::<serde_json::Map<String, serde_json::Value>>();
            let obj_value = serde_json::Value::Object(json_obj);
            let desc_json = serde_json::to_string(&obj_value).unwrap_or_default();
            format!("{default_message}#{desc_json}")
        } else {
            default_message.clone()
        };
        interpolate_name(filename, interpolate_pattern, &content)
    } else {
        id
    };

    MessageDescriptor {
        id,
        default_message: Some(default_message),
        description,
    }
}

fn store_message(
    messages: &mut Vec<ExtractedMessage>,
    descriptor: &MessageDescriptor,
    filename: &str,
    location: Option<(Loc, Loc)>,
) {
    if descriptor.id.is_none() && descriptor.default_message.is_none() {
        let handler = &swc_core::plugin::errors::HANDLER;

        handler.with(|handler| {
            handler
                .struct_err("[React Intl] Message Descriptors require an `id` or `defaultMessage`.")
                .emit()
        });
    }

    let source_location = if let Some(location) = location {
        let (start, end) = location;

        // NOTE: this is not fully identical to babel's test snapshot output
        Some(SourceLocation {
            file: filename.to_string(),
            start: Location {
                line: start.line,
                col: start.col.to_usize(),
            },
            end: Location {
                line: end.line,
                col: end.col.to_usize(),
            },
        })
    } else {
        None
    };

    messages.push(ExtractedMessage {
        id: descriptor
            .id
            .as_ref()
            .unwrap_or(&"".to_string())
            .to_string(),
        default_message: descriptor
            .default_message
            .as_ref()
            .expect("Should be available")
            .clone(),
        description: descriptor.description.clone(),
        loc: source_location,
    });
}

fn get_message_object_from_expression(expr: Option<&mut ExprOrSpread>) -> Option<&mut Expr> {
    if let Some(expr) = expr {
        let expr = &mut *expr.expr;
        Some(expr)
    } else {
        None
    }
}

fn assert_object_expression(expr: &Option<&mut Expr>, callee: &Callee) {
    let assert_fail = match expr {
        Some(expr) => !expr.is_object(),
        _ => true,
    };

    if assert_fail {
        let prop = if let Callee::Expr(expr) = callee {
            if let Expr::Ident(ident) = &**expr {
                Some(ident.sym.to_string())
            } else {
                None
            }
        } else {
            None
        };

        let handler = &swc_core::plugin::errors::HANDLER;

        handler.with(|handler| {
            handler
                .struct_err(
                    &(format!(
                        r#"[React Intl] `{}` must be called with an object expression
                        with values that are React Intl Message Descriptors,
                        also defined as object expressions."#,
                        prop.unwrap_or_default()
                    )),
                )
                .emit()
        });
    }
}

fn evaluate_template_literal_string(tpl: &Tpl) -> String {
    //NOTE: This doesn't fully evaluate templates
    tpl.quasis
        .iter()
        .map(|q| {
            q.cooked
                .as_ref()
                .map(|v| v.to_string_lossy().to_string())
                .unwrap_or_default()
        })
        .collect::<Vec<String>>()
        .join("")
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ExtractedMessage {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<MessageDescriptionValue>,
    pub default_message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loc: Option<SourceLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceLocation {
    pub file: String,
    pub start: Location,
    pub end: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

pub struct FormatJSVisitor<C: Clone + Comments, S: SourceMapper> {
    // We may not need Arc in the plugin context - this is only to preserve isomorphic interface
    // between plugin & custom transform pass.
    source_map: std::sync::Arc<S>,
    comments: C,
    options: FormatJSPluginOptions,
    filename: String,
    messages: Vec<ExtractedMessage>,
    meta: HashMap<String, String>,
    component_names: HashSet<String>,
    function_names: HashSet<String>,
    // Variable tracking for React Compiler optimizations
    variable_bindings: HashMap<Id, Expr>,
}

impl<C: Clone + Comments, S: SourceMapper> FormatJSVisitor<C, S> {
    fn new(
        source_map: std::sync::Arc<S>,
        comments: C,
        plugin_options: FormatJSPluginOptions,
        filename: &str,
    ) -> Self {
        let mut function_names: HashSet<String> = Default::default();
        plugin_options
            .additional_function_names
            .iter()
            .for_each(|name| {
                function_names.insert(name.to_string());
            });
        function_names.insert("formatMessage".to_string());
        function_names.insert("$formatMessage".to_string());

        let mut component_names: HashSet<String> = Default::default();
        component_names.insert("FormattedMessage".to_string());
        plugin_options
            .additional_component_names
            .iter()
            .for_each(|name| {
                component_names.insert(name.to_string());
            });

        FormatJSVisitor {
            source_map,
            comments,
            options: plugin_options,
            filename: filename.to_string(),
            messages: Default::default(),
            meta: Default::default(),
            component_names,
            function_names,
            variable_bindings: Default::default(),
        }
    }

    fn read_pragma(&mut self, span_lo: BytePos, span_hi: BytePos) {
        if let Some(pragma) = &self.options.pragma {
            let mut comments = self.comments.get_leading(span_lo).unwrap_or_default();
            comments.append(&mut self.comments.get_leading(span_hi).unwrap_or_default());

            let pragma = pragma.as_str();

            for comment in comments {
                let comment_text = &*comment.text;
                if comment_text.contains(pragma) {
                    let value = comment_text.split(pragma).nth(1);
                    if let Some(value) = value {
                        let value = WHITESPACE_REGEX.split(value.trim());
                        for kv in value {
                            let mut kv = kv.split(":");
                            if let Some(k) = kv.next() {
                                if let Some(v) = kv.next() {
                                    self.meta.insert(k.to_string(), v.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn resolve_identifier(&self, ident: &swc_core::ecma::ast::Ident) -> Option<&Expr> {
        self.variable_bindings.get(&ident.to_id())
    }

    fn get_jsx_message_descriptor_value_maybe_object_with_resolution(
        &self,
        value: Option<&JSXAttrValue>,
        is_message_node: Option<bool>,
    ) -> Option<MessageDescriptionValue> {
        let value = value?;
        // NOTE: do not support evaluatePath
        match value {
            JSXAttrValue::JSXExprContainer(container) => {
                if is_message_node.unwrap_or(false) {
                    if let JSXExpr::Expr(expr) = &container.expr {
                        // If this is already compiled, no need to recompiled it
                        if let Expr::Array(..) = &**expr {
                            return None;
                        }
                    }
                }

                match &container.expr {
                    JSXExpr::Expr(expr) => match &**expr {
                        Expr::Lit(Lit::Str(s)) => Some(MessageDescriptionValue::Str(
                            s.value.to_string_lossy().into_owned(),
                        )),
                        Expr::Object(object_lit) => {
                            Some(MessageDescriptionValue::Obj(object_lit.clone()))
                        }
                        Expr::Tpl(tpl) => Some(MessageDescriptionValue::Str(
                            evaluate_template_literal_string(tpl),
                        )),
                        Expr::Bin(bin_expr) => self.evaluate_binary_expr_with_resolution(bin_expr),
                        // Handle React Compiler optimized identifiers
                        Expr::Ident(ident) => {
                            if let Some(resolved_expr) = self.resolve_identifier(ident) {
                                match resolved_expr {
                                    Expr::Object(object_lit) => {
                                        Some(MessageDescriptionValue::Obj(object_lit.clone()))
                                    }
                                    Expr::Lit(Lit::Str(s)) => Some(MessageDescriptionValue::Str(
                                        s.value.to_string_lossy().into_owned(),
                                    )),
                                    Expr::Tpl(tpl) => Some(MessageDescriptionValue::Str(
                                        evaluate_template_literal_string(tpl),
                                    )),
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                    _ => None,
                }
            }
            JSXAttrValue::Str(s) => Some(MessageDescriptionValue::Str(
                s.value.to_atom_lossy().to_string(),
            )),
            _ => None,
        }
    }

    /// Helper method to evaluate binary expressions with resolution support
    fn evaluate_binary_expr_with_resolution(
        &self,
        expr: &BinExpr,
    ) -> Option<MessageDescriptionValue> {
        // Only handle string concatenation (+ operator)
        if !matches!(expr.op, BinaryOp::Add) {
            return None;
        }

        // Recursively evaluate left and right operands
        let left_val = self.get_call_expr_message_descriptor_value_maybe_object_with_resolution(
            Some(&*expr.left),
            None,
        )?;
        let right_val = self.get_call_expr_message_descriptor_value_maybe_object_with_resolution(
            Some(&*expr.right),
            None,
        )?;

        // Only concatenate if both are strings
        match (left_val, right_val) {
            (MessageDescriptionValue::Str(left_str), MessageDescriptionValue::Str(right_str)) => {
                Some(MessageDescriptionValue::Str(format!(
                    "{left_str}{right_str}"
                )))
            }
            _ => None,
        }
    }

    fn get_call_expr_message_descriptor_value_maybe_object_with_resolution(
        &self,
        value: Option<&Expr>,
        _is_message_node: Option<bool>,
    ) -> Option<MessageDescriptionValue> {
        let value = value?;
        // NOTE: do not support evaluatePath
        match value {
            Expr::Ident(ident) => {
                // First try to resolve the identifier to see if it's a variable reference
                if let Some(resolved_expr) = self.resolve_identifier(ident) {
                    match resolved_expr {
                        Expr::Object(object_lit) => {
                            Some(MessageDescriptionValue::Obj(object_lit.clone()))
                        }
                        Expr::Lit(Lit::Str(s)) => Some(MessageDescriptionValue::Str(
                            s.value.to_atom_lossy().to_string(),
                        )),
                        _ => None,
                    }
                } else {
                    // Fall back to treating identifier as a string value
                    Some(MessageDescriptionValue::Str(ident.sym.to_string()))
                }
            }
            Expr::Lit(Lit::Str(s)) => Some(MessageDescriptionValue::Str(
                s.value.to_atom_lossy().to_string(),
            )),
            Expr::Object(object_lit) => Some(MessageDescriptionValue::Obj(object_lit.clone())),
            Expr::Bin(bin_expr) => self.evaluate_binary_expr_with_resolution(bin_expr),
            _ => None,
        }
    }

    fn process_message_object(&mut self, message_descriptor: &mut Option<&mut Expr>) {
        if let Some(message_obj) = &mut *message_descriptor {
            let (lo, hi) = (message_obj.span().lo, message_obj.span().hi);

            if let Expr::Object(obj) = *message_obj {
                let properties = &obj.props;

                let descriptor_path = create_message_descriptor_from_call_expr(properties);

                // If the message is already compiled, don't re-compile it
                if let Some(default_message) = &descriptor_path.default_message {
                    if default_message.is_array() {
                        return;
                    }
                }

                let descriptor = evaluate_call_expr_message_descriptor_with_visitor(
                    &descriptor_path,
                    &self.options,
                    &self.filename,
                    self,
                );

                let source_location = if self.options.extract_source_location {
                    Some((
                        self.source_map.lookup_char_pos(lo),
                        self.source_map.lookup_char_pos(hi),
                    ))
                } else {
                    None
                };

                store_message(
                    &mut self.messages,
                    &descriptor,
                    &self.filename,
                    source_location,
                );

                // let first_prop = properties.first().is_some();

                // Insert ID potentially 1st before removing nodes
                let id_prop = obj.props.iter().find(|prop| {
                    if let PropOrSpread::Prop(prop) = prop {
                        if let Prop::KeyValue(kv) = &**prop {
                            return match &kv.key {
                                PropName::Ident(ident) => &*ident.sym == "id",
                                PropName::Str(str_) => &*str_.value == "id",
                                _ => false,
                            };
                        }
                    }
                    false
                });

                if let Some(descriptor_id) = descriptor.id {
                    if let Some(id_prop) = id_prop {
                        let prop = id_prop.as_prop().unwrap();
                        let kv = &mut prop.as_key_value().unwrap();
                        kv.to_owned().value = Box::new(Expr::Lit(Lit::Str(Str {
                            span: DUMMY_SP,
                            value: descriptor_id.into(),
                            raw: None,
                        })));
                    } else {
                        obj.props.insert(
                            0,
                            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                key: PropName::Ident(IdentName::new("id".into(), DUMMY_SP)),
                                value: Box::new(Expr::Lit(Lit::Str(Str {
                                    span: DUMMY_SP,
                                    value: descriptor_id.into(),
                                    raw: None,
                                }))),
                            }))),
                        )
                    }
                }

                let mut props = vec![];
                for prop in obj.props.drain(..) {
                    match prop {
                        PropOrSpread::Prop(mut prop) => {
                            if let Prop::KeyValue(keyvalue) = &mut *prop {
                                let key = get_message_descriptor_key_from_call_expr(&keyvalue.key);
                                if let Some(key) = key {
                                    match key {
                                        "description" => {
                                            // remove description
                                            if descriptor.description.is_some() {
                                                self.comments.take_leading(prop.span().lo);
                                            } else {
                                                props.push(PropOrSpread::Prop(prop));
                                            }
                                        }
                                        // Pre-parse or remove defaultMessage
                                        "defaultMessage" => {
                                            if self.options.remove_default_message {
                                                // remove defaultMessage
                                            } else {
                                                if let Some(descriptor_default_message) =
                                                    descriptor.default_message.as_ref()
                                                {
                                                    if self.options.ast {
                                                        if let Ok(parsed_expr) = parse(
                                                            descriptor_default_message.as_str(),
                                                        ) {
                                                            keyvalue.value = parsed_expr;
                                                        }
                                                    } else {
                                                        keyvalue.value =
                                                            Box::new(Expr::Lit(Lit::Str(Str {
                                                                span: DUMMY_SP,
                                                                value: descriptor_default_message
                                                                    .as_str()
                                                                    .into(),
                                                                raw: None,
                                                            })));
                                                    }
                                                }

                                                props.push(PropOrSpread::Prop(prop));
                                            }
                                        }
                                        _ => props.push(PropOrSpread::Prop(prop)),
                                    }
                                } else {
                                    props.push(PropOrSpread::Prop(prop));
                                }
                            } else {
                                props.push(PropOrSpread::Prop(prop));
                            }
                        }
                        _ => props.push(prop),
                    }
                }

                obj.props = props;
            }
        }
    }
}

impl<C: Clone + Comments, S: SourceMapper> VisitMut for FormatJSVisitor<C, S> {
    noop_visit_mut_type!(fail);

    fn visit_mut_var_declarator(&mut self, var_declarator: &mut VarDeclarator) {
        var_declarator.visit_mut_children_with(self);

        // Track variable declarations for React Compiler optimizations
        if let (Pat::Ident(binding_ident), Some(init)) =
            (&var_declarator.name, &var_declarator.init)
        {
            // Store the variable binding
            self.variable_bindings
                .insert(binding_ident.id.to_id(), *init.clone());
        }
    }

    fn visit_mut_assign_expr(&mut self, assign_expr: &mut AssignExpr) {
        assign_expr.visit_mut_children_with(self);

        // Track assignment expressions for React Compiler optimizations
        // Handle patterns like: t1 = { ... }
        if let AssignTarget::Simple(SimpleAssignTarget::Ident(ident)) = &assign_expr.left {
            let variable_id = ident.id.to_id();

            // Check if we already have a binding for this variable
            let should_update = match self.variable_bindings.get(&variable_id) {
                Some(existing_expr) => {
                    // Only overwrite if the new expression is an object literal
                    // and the existing one is not, or if both are object literals
                    match (existing_expr, &*assign_expr.right) {
                        (Expr::Object(_), Expr::Object(_)) => true, // Both objects, update
                        (_, Expr::Object(_)) => true,               /* New is object, existing */
                        // is not, update
                        (Expr::Object(_), _) => false, /* Existing is object, new is not, don't */
                        // update
                        _ => true, // Neither is object, update
                    }
                }
                None => true, // No existing binding, always update
            };

            if should_update {
                self.variable_bindings
                    .insert(variable_id, *assign_expr.right.clone());
            }
        }
    }

    fn visit_mut_jsx_opening_element(&mut self, jsx_opening_elem: &mut JSXOpeningElement) {
        jsx_opening_elem.visit_mut_children_with(self);

        let name = &jsx_opening_elem.name;

        if let JSXElementName::Ident(ident) = name {
            if !self.component_names.contains(&*ident.sym) {
                return;
            }
        }

        let descriptor_path = create_message_descriptor_from_jsx_attr(&jsx_opening_elem.attrs);

        // In order for a default message to be extracted when
        // declaring a JSX element, it must be done with standard
        // `key=value` attributes. But it's completely valid to
        // write `<FormattedMessage {...descriptor} />`, because it will be
        // skipped here and extracted elsewhere. The descriptor will
        // be extracted only (storeMessage) if a `defaultMessage` prop.
        if descriptor_path.default_message.is_none() {
            return;
        }

        // Evaluate the Message Descriptor values in a JSX
        // context, then store it.
        let descriptor = evaluate_jsx_message_descriptor_with_visitor(
            &descriptor_path,
            &self.options,
            &self.filename,
            self,
        );

        let source_location = if self.options.extract_source_location {
            Some((
                self.source_map.lookup_char_pos(jsx_opening_elem.span().lo),
                self.source_map.lookup_char_pos(jsx_opening_elem.span().hi),
            ))
        } else {
            None
        };

        store_message(
            &mut self.messages,
            &descriptor,
            &self.filename,
            source_location,
        );

        let id_attr = jsx_opening_elem.attrs.iter().find(|attr| match attr {
            JSXAttrOrSpread::JSXAttr(attr) => {
                if let JSXAttrName::Ident(ident) = &attr.name {
                    &*ident.sym == "id"
                } else {
                    false
                }
            }
            _ => false,
        });

        let first_attr = !jsx_opening_elem.attrs.is_empty();

        // Do not support overrideIdFn, only support idInterpolatePattern
        if descriptor.id.is_some() {
            if let Some(id_attr) = id_attr {
                if let JSXAttrOrSpread::JSXAttr(attr) = id_attr {
                    attr.to_owned().value =
                        Some(JSXAttrValue::Str(Str::from(descriptor.id.unwrap().clone())));
                }
            } else if first_attr {
                jsx_opening_elem.attrs.insert(
                    0,
                    JSXAttrOrSpread::JSXAttr(JSXAttr {
                        span: DUMMY_SP,
                        name: JSXAttrName::Ident(IdentName::new("id".into(), DUMMY_SP)),
                        value: Some(JSXAttrValue::Str(Str::from(descriptor.id.unwrap()))),
                    }),
                )
            }
        }

        let mut attrs = vec![];
        for attr in jsx_opening_elem.attrs.drain(..) {
            match attr {
                JSXAttrOrSpread::JSXAttr(attr) => {
                    let key = get_message_descriptor_key_from_jsx(&attr.name);
                    match key {
                        "description" => {
                            // remove description
                            if descriptor.description.is_some() {
                                self.comments.take_leading(attr.span.lo);
                            } else {
                                attrs.push(JSXAttrOrSpread::JSXAttr(attr));
                            }
                        }
                        "defaultMessage" => {
                            if self.options.remove_default_message {
                                // remove defaultMessage
                            } else {
                                let mut attr = attr.to_owned();
                                if let Some(descriptor_default_message) =
                                    descriptor.default_message.as_ref()
                                {
                                    if self.options.ast {
                                        if let Ok(parsed_expr) =
                                            parse(descriptor_default_message.as_str())
                                        {
                                            attr.value = Some(JSXAttrValue::JSXExprContainer(
                                                JSXExprContainer {
                                                    span: DUMMY_SP,
                                                    expr: JSXExpr::Expr(parsed_expr),
                                                },
                                            ));
                                        }
                                    } else {
                                        // Only update the defaultMessage value with the evaluated
                                        // string
                                        // if the original value was a binary expression
                                        // (concatenation)
                                        // Otherwise, keep the original to preserve formatting
                                        let should_update = if let Some(
                                            JSXAttrValue::JSXExprContainer(container),
                                        ) = &attr.value
                                        {
                                            if let JSXExpr::Expr(expr) = &container.expr {
                                                matches!(&**expr, Expr::Bin(_))
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        };

                                        if should_update {
                                            attr.value = Some(JSXAttrValue::Str(Str::from(
                                                descriptor_default_message.clone(),
                                            )));
                                        }
                                    }
                                }
                                attrs.push(JSXAttrOrSpread::JSXAttr(attr))
                            }
                        }
                        _ => attrs.push(JSXAttrOrSpread::JSXAttr(attr)),
                    }
                }
                _ => attrs.push(attr),
            }
        }

        jsx_opening_elem.attrs = attrs.to_vec();

        // tag_as_extracted();
    }

    fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
        call_expr.visit_mut_children_with(self);

        let callee = &call_expr.callee;
        let args = &mut call_expr.args;

        if let Callee::Expr(callee_expr) = callee {
            if let Expr::Ident(ident) = &**callee_expr {
                if &*ident.sym == "defineMessage" || &*ident.sym == "defineMessages" {
                    let first_arg = args.get_mut(0);
                    let mut message_obj = get_message_object_from_expression(first_arg);

                    assert_object_expression(&message_obj, callee);

                    if &*ident.sym == "defineMessage" {
                        self.process_message_object(&mut message_obj);
                    } else if let Some(Expr::Object(obj)) = message_obj {
                        for prop in obj.props.iter_mut() {
                            if let PropOrSpread::Prop(prop) = &mut *prop {
                                if let Prop::KeyValue(kv) = &mut **prop {
                                    self.process_message_object(&mut Some(&mut *kv.value));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Check that this is `intl.formatMessage` call
        if let Callee::Expr(expr) = &callee {
            let is_format_message_call = match &**expr {
                Expr::Ident(ident) if self.function_names.contains(&*ident.sym) => true,
                Expr::Member(member_expr) => {
                    if let MemberProp::Ident(ident) = &member_expr.prop {
                        self.function_names.contains(&*ident.sym)
                    } else {
                        false
                    }
                }
                _ => false,
            };

            if is_format_message_call {
                let message_descriptor = args.get_mut(0);
                if let Some(message_descriptor) = message_descriptor {
                    if message_descriptor.expr.is_object() {
                        self.process_message_object(&mut Some(message_descriptor.expr.as_mut()));
                    }
                }
            }
        }
    }

    fn visit_mut_module_items(&mut self, items: &mut Vec<ModuleItem>) {
        /*
        if self.is_instrumented_already() {
            return;
        }
        */

        for item in items {
            self.read_pragma(item.span().lo, item.span().hi);
            item.visit_mut_children_with(self);
        }

        if self.options.__debug_extracted_messages_comment {
            let messages_json_str =
                serde_json::to_string(&self.messages).expect("Should be serializable");
            let meta_json_str = serde_json::to_string(&self.meta).expect("Should be serializable");

            // Append extracted messages to the end of the file as stringified JSON
            // comments. SWC's plugin does not support to return aribitary data
            // other than transformed codes, There's no way to pass extracted
            // messages after transform. This is not a public interface;
            // currently for debugging / testing purpose only.
            self.comments.add_trailing(
                Span::dummy_with_cmt().hi,
                Comment {
                    kind: CommentKind::Block,
                    span: Span::dummy_with_cmt(),
                    text: format!(
                        "__formatjs__messages_extracted__::{{\"messages\":{messages_json_str}, \
                         \"meta\":{meta_json_str}}}"
                    )
                    .into(),
                },
            );
        }
    }
}

fn json_value_to_expr(json_value: &serde_json::Value) -> Box<Expr> {
    Box::new(match json_value {
        serde_json::Value::Null => {
            Expr::Lit(Lit::Null(swc_core::ecma::ast::Null { span: DUMMY_SP }))
        }
        serde_json::Value::Bool(v) => Expr::Lit(Lit::Bool(Bool {
            span: DUMMY_SP,
            value: *v,
        })),
        serde_json::Value::Number(v) => Expr::Lit(Lit::Num(Number {
            span: DUMMY_SP,
            raw: None,
            value: v.as_f64().unwrap(),
        })),
        serde_json::Value::String(v) => Expr::Lit(Lit::Str(Str {
            span: DUMMY_SP,
            raw: None,
            value: v.as_str().into(),
        })),
        serde_json::Value::Array(v) => Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: v
                .iter()
                .map(|elem| {
                    Some(ExprOrSpread {
                        spread: None,
                        expr: json_value_to_expr(elem),
                    })
                })
                .collect(),
        }),
        serde_json::Value::Object(v) => Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: v
                .iter()
                .map(|(key, value)| {
                    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                        key: PropName::Str(Str::from(key.clone())),
                        value: json_value_to_expr(value),
                    })))
                })
                .collect(),
        }),
    })
}

pub fn create_formatjs_visitor<C: Clone + Comments, S: SourceMapper>(
    source_map: std::sync::Arc<S>,
    comments: C,
    plugin_options: FormatJSPluginOptions,
    filename: &str,
) -> FormatJSVisitor<C, S> {
    FormatJSVisitor::new(source_map, comments, plugin_options, filename)
}

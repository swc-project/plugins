use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::Arc,
};

use base64::Engine;
use fxhash::FxHashMap;
use import_map::ImportMap;
use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use sourcemap::{RawToken, SourceMap as RawSourcemap};
use swc_atoms::JsWord;
use swc_common::{comments::Comments, util::take::Take, BytePos, SourceMapperDyn, DUMMY_SP};
use swc_ecma_ast::{
    ArrayLit, CallExpr, Callee, ClassDecl, ClassMethod, ClassProp, Expr, ExprOrSpread, FnDecl, Id,
    Ident, ImportDecl, ImportSpecifier, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue,
    JSXElement, JSXElementName, JSXExpr, JSXExprContainer, JSXObject, KeyValueProp, MemberProp,
    MethodProp, ModuleExportName, ObjectLit, Pat, Prop, PropName, PropOrSpread, SourceMapperExt,
    SpreadElement, Tpl, VarDeclarator,
};
use swc_ecma_utils::ExprFactory;
use swc_ecma_visit::{Fold, FoldWith};
use swc_trace_macro::swc_trace;

mod import_map;

static EMOTION_OFFICIAL_LIBRARIES: Lazy<Vec<EmotionModuleConfig>> = Lazy::new(|| {
    vec![
        EmotionModuleConfig {
            module_name: "@emotion/css".into(),
            exported_names: vec![ExportItem {
                name: "css".to_owned(),
                kind: ExprKind::Css,
            }],
            default_export: Some(ExprKind::Css),
        },
        EmotionModuleConfig {
            module_name: "@emotion/styled".into(),
            exported_names: vec![],
            default_export: Some(ExprKind::Styled),
        },
        EmotionModuleConfig {
            module_name: "@emotion/react".into(),
            exported_names: vec![
                ExportItem {
                    name: "css".to_owned(),
                    kind: ExprKind::Css,
                },
                ExportItem {
                    name: "keyframes".to_owned(),
                    kind: ExprKind::Css,
                },
                ExportItem {
                    name: "Global".to_owned(),
                    kind: ExprKind::GlobalJSX,
                },
            ],
            ..Default::default()
        },
        EmotionModuleConfig {
            module_name: "@emotion/primitives".into(),
            exported_names: vec![ExportItem {
                name: "css".to_owned(),
                kind: ExprKind::Css,
            }],
            default_export: Some(ExprKind::Styled),
        },
        EmotionModuleConfig {
            module_name: "@emotion/native".into(),
            exported_names: vec![ExportItem {
                name: "css".to_owned(),
                kind: ExprKind::Css,
            }],
            default_export: Some(ExprKind::Styled),
        },
    ]
});

static INVALID_LABEL_SPACES: Lazy<Regex> = Lazy::new(|| RegexBuilder::new(r"\s+").build().unwrap());

static INVALID_CSS_CLASS_NAME_CHARACTERS: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(r##"[!"#$%&'()*+,./:;<=>?@\[\]^`|}~{]"##)
        .build()
        .unwrap()
});

static INVALID_SINGLE_LINE_COMMENT: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(r#"(?P<s>^|[^:^'^"]|\s)//.*$"#)
        .multi_line(true)
        .build()
        .unwrap()
});

static MULTI_LINE_COMMENT: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(r"(?s)/\*.*?\*/")
        .multi_line(true)
        .build()
        .unwrap()
});

static SPACE_AROUND_COLON: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s*(?P<s>[:;,\{,\}])\s*").unwrap());

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmotionOptions {
    pub enabled: Option<bool>,
    pub sourcemap: Option<bool>,
    pub auto_label: Option<bool>,
    pub label_format: Option<String>,
    pub import_map: Option<ImportMap>,
}

impl Default for EmotionOptions {
    fn default() -> Self {
        EmotionOptions {
            enabled: Some(false),
            sourcemap: Some(true),
            auto_label: Some(true),
            label_format: Some("[local]".to_owned()),
            import_map: None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmotionModuleConfig {
    module_name: JsWord,
    exported_names: Vec<ExportItem>,
    default_export: Option<ExprKind>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct ExportItem {
    name: String,
    kind: ExprKind,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
enum ImportType {
    #[default]
    Named,
    Namespace,
    Default,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
enum ExprKind {
    #[default]
    Css,
    Styled,
    GlobalJSX,
}

#[derive(Debug)]
enum PackageMeta {
    Named(ExprKind),
    Namespace(EmotionModuleConfig),
}

pub fn emotion<C: Comments>(
    emotion_options: EmotionOptions,
    path: &Path,
    src_file_hash: u32,
    cm: Arc<SourceMapperDyn>,
    comments: C,
) -> impl Fold {
    EmotionTransformer::new(emotion_options, path, src_file_hash, cm, comments)
}

pub struct EmotionTransformer<C: Comments> {
    pub options: EmotionOptions,
    filepath_hash: Option<u32>,
    filepath: PathBuf,
    dirname: Option<String>,
    filename: Option<String>,
    src_file_hash: u32,
    cm: Arc<SourceMapperDyn>,
    comments: C,
    import_packages: FxHashMap<Id, PackageMeta>,
    emotion_target_class_name_count: usize,
    current_context: Option<String>,
    current_class: Option<String>,
    // skip `css` transformation if it in JSX Element/Attribute
    in_jsx_element: bool,

    registered_imports: Vec<EmotionModuleConfig>,
}

#[swc_trace]
impl<C: Comments> EmotionTransformer<C> {
    pub fn new(
        options: EmotionOptions,
        path: &Path,
        src_file_hash: u32,
        cm: Arc<SourceMapperDyn>,
        comments: C,
    ) -> Self {
        let registered_imports = self::import_map::expand_import_map(
            options.import_map.as_ref(),
            EMOTION_OFFICIAL_LIBRARIES.to_vec(),
        );

        EmotionTransformer {
            options,
            filepath_hash: None,
            filepath: path.to_owned(),
            src_file_hash,
            dirname: path
                .parent()
                .and_then(|parent| parent.file_name())
                .and_then(|dirname| dirname.to_str())
                .map(|s| s.to_owned()),
            filename: path
                .file_stem()
                .and_then(|filename| filename.to_str())
                .and_then(|s| {
                    s.rfind('\\')
                        .map(|pos| &s[pos + 1..]) // if backslashes are found, take the last part
                        .or(Some(s))              // otherwise use the whole path
                    })
                .map(|s| s.to_owned()),
            cm,
            comments,
            import_packages: FxHashMap::default(),
            emotion_target_class_name_count: 0,
            current_context: None,
            current_class: None,
            in_jsx_element: false,
            registered_imports,
        }
    }

    fn sanitize_label_part<'t>(&self, label_part: &'t str) -> String {
        // Existing @emotion/babel-plugin behaviour is to replace all spaces
        // with a single hyphen
        let without_spaces = INVALID_LABEL_SPACES.replace_all(label_part, "-");
        INVALID_CSS_CLASS_NAME_CHARACTERS
            .replace_all(&without_spaces, "-")
            .to_string()
    }

    fn create_label(&self, with_prefix: bool) -> String {
        let prefix = if with_prefix { "label:" } else { "" };
        let mut label = format!(
            "{}{}",
            prefix,
            self.options
                .label_format
                .clone()
                .unwrap_or_else(|| "[local]".to_owned())
        );
        if let Some(current_context) = &self.current_context {
            label = label.replace("[local]", &self.sanitize_label_part(current_context));
            if let Some(filename) = self.filename.as_ref() {
                label = label.replace("[filename]", &self.sanitize_label_part(filename));
            }
            if let Some(dirname) = self.dirname.as_ref() {
                label = label.replace("[dirname]", &self.sanitize_label_part(dirname));
            };
        } else {
            // Existing @emotion/babel-plugin behaviour is to
            // not provide a label if there is no available identifier
            return "".to_string();
        }
        label
    }

    fn create_sourcemap(&mut self, pos: BytePos) -> Option<String> {
        if self.options.sourcemap.unwrap_or(false) {
            let loc = self.cm.get_code_map().lookup_char_pos(pos);
            let filename = self.filepath.to_str().map(Arc::<str>::from);
            let cm = RawSourcemap::new(
                filename.clone(),
                vec![RawToken {
                    dst_line: 0,
                    dst_col: 0,
                    src_line: loc.line as u32 - 1,
                    src_col: loc.col_display as u32,
                    src_id: 0,
                    name_id: 0,
                    is_range: false,
                }],
                Vec::new(),
                vec![filename.unwrap_or_else(|| Arc::from(""))],
                Some(vec![Some(loc.file.src.to_string().into())]),
            );
            let mut writer = Vec::new();
            if cm.to_writer(&mut writer).is_ok() {
                return Some(format!(
                    "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,{} */",
                    base64::engine::general_purpose::STANDARD.encode(writer)
                ));
            }
        }
        None
    }

    // Find the imported name from modules
    // These import statements are supported:
    //    import styled from '@emotion/styled'
    //    import { default as whateverStyled } from '@emotion/styled'
    //    import { css } from '@emotion/react'
    //    import * as emotionCss from '@emotion/react'
    fn generate_import_info(&mut self, expr: &ImportDecl) {
        for c in self.registered_imports.iter() {
            if expr.src.value == c.module_name {
                for specifier in expr.specifiers.iter() {
                    match specifier {
                        ImportSpecifier::Named(named) => {
                            for exported in c.exported_names.iter() {
                                let matched = match &named.imported {
                                    Some(imported) => match imported {
                                        ModuleExportName::Ident(v) => v.sym == exported.name,
                                        ModuleExportName::Str(v) => v.value == exported.name,
                                    },
                                    _ => named.local.as_ref() == exported.name,
                                };
                                if matched {
                                    self.import_packages.insert(
                                        named.local.to_id(),
                                        PackageMeta::Named(exported.kind),
                                    );
                                }
                            }
                        }
                        ImportSpecifier::Default(default) => {
                            if let Some(kind) = c.default_export {
                                self.import_packages
                                    .insert(default.local.to_id(), PackageMeta::Named(kind));
                            }
                        }
                        ImportSpecifier::Namespace(namespace) => {
                            self.import_packages
                                .insert(namespace.local.to_id(), PackageMeta::Namespace(c.clone()));
                        }
                    }
                }
            }
        }
    }

    fn create_label_prop_node(&mut self, key: &str) -> PropOrSpread {
        let stable_class_name = format!(
            "e{}{}",
            radix_fmt::radix_36(self.src_file_hash),
            self.emotion_target_class_name_count
        );
        self.emotion_target_class_name_count += 1;
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(Ident::new(key.into(), DUMMY_SP)),
            value: stable_class_name.into(),
        })))
    }

    fn create_args_from_tagged_tpl(&self, tagged_tpl: &mut Tpl) -> Vec<ExprOrSpread> {
        let args_len = tagged_tpl.exprs.len() + tagged_tpl.quasis.len();
        // 2 more capacity is for `label` and `sourceMap`
        let mut args = Vec::with_capacity(args_len + 2);
        for index in 0..args_len {
            let i = index / 2;
            if index % 2 == 0 {
                if let Some(q) = tagged_tpl.quasis.get_mut(i) {
                    let q = q.take();
                    let css_input = q
                        .raw
                        .replace("\\`", "`")
                        .replace("\\$", "$")
                        .replace("\\b", "\u{0008}")
                        .replace("\\f", "\u{000C}")
                        .replace("\\n", "\n")
                        .replace("\\r", "\r")
                        .replace("\\t", "\t")
                        .replace("\\v", "\u{000B}")
                        .replace("\\\\", "\\");

                    let minified = minify_css_string(&css_input, index == 0, index == args_len - 1);
                    // Compress one more spaces into one space
                    if minified.replace(' ', "").is_empty() {
                        if index != 0 && index != args_len - 1 {
                            args.push(" ".as_arg());
                        }
                    } else {
                        args.push(minified.as_arg())
                    }
                }
            } else if let Some(e) = tagged_tpl.exprs.get_mut(i) {
                args.push(e.take().as_arg());
            }
        }
        args
    }

    fn rewrite_styles_attr(&mut self, attrs: &mut [JSXAttrOrSpread], pos: BytePos) {
        if let Some(attr_value) = attrs.iter_mut().find_map(|attr| {
            if let JSXAttrOrSpread::JSXAttr(JSXAttr {
                name: JSXAttrName::Ident(i),
                value,
                ..
            }) = attr
            {
                if i.as_ref() == "styles" {
                    return value.as_mut();
                }
            }
            None
        }) {
            if let Some(raw_attr) = match attr_value {
                JSXAttrValue::Lit(lit) => Some(Box::new(Expr::Lit(lit.clone()))),
                JSXAttrValue::JSXExprContainer(JSXExprContainer {
                    expr: JSXExpr::Expr(expr),
                    ..
                }) => Some(expr.take()),
                _ => None,
            } {
                *attr_value = self.create_styles_attr(raw_attr, pos);
                self.in_jsx_element = true;
            }
        }
    }

    fn create_styles_attr(&mut self, mut raw_attr: Box<Expr>, pos: BytePos) -> JSXAttrValue {
        if let Expr::Array(array_lit) = raw_attr.as_mut() {
            if let Some(cm) = self.create_sourcemap(pos) {
                array_lit.elems.push(Some(cm.as_arg()));
            }
            JSXAttrValue::JSXExprContainer(JSXExprContainer {
                span: DUMMY_SP,
                expr: JSXExpr::Expr(raw_attr),
            })
        } else {
            JSXAttrValue::JSXExprContainer(JSXExprContainer {
                span: DUMMY_SP,
                expr: JSXExpr::Expr(Box::new(Expr::Array(ArrayLit {
                    span: DUMMY_SP,
                    elems: {
                        let mut elements = Vec::with_capacity(2);
                        elements.push(Some(raw_attr.as_arg()));
                        if let Some(cm) = self.create_sourcemap(pos) {
                            elements.push(Some(cm.as_arg()));
                        }
                        elements
                    },
                }))),
            })
        }
    }
}

impl<C: Comments> Fold for EmotionTransformer<C> {
    fn fold_call_expr(&mut self, mut expr: CallExpr) -> CallExpr {
        // If no package that we care about is imported, skip the following
        // transformation logic.
        if self.import_packages.is_empty() {
            return expr;
        }
        if let Callee::Expr(e) = &mut expr.callee {
            match e.as_mut() {
                // css({})
                Expr::Ident(i) => {
                    if let Some(package) = self.import_packages.get(&i.to_id()) {
                        if !expr.args.is_empty() {
                            if let PackageMeta::Named(kind) = package {
                                if matches!(kind, ExprKind::Css) && !self.in_jsx_element {
                                    self.comments.add_pure_comment(expr.span.lo());
                                    if self.options.auto_label.unwrap_or(false) {
                                        expr.args.push(self.create_label(true).as_arg());
                                    }
                                    if let Some(cm) = self.create_sourcemap(expr.span.lo) {
                                        expr.args.push(cm.as_arg());
                                    }
                                }
                            }
                        }
                    } else {
                        // Make sure we hit the get the children of the expression too.
                        // https://github.com/swc-project/plugins/issues/303
                        return expr.fold_children_with(self);
                    }
                }
                // styled('div')({})
                Expr::Call(c) => {
                    if let Callee::Expr(callee_exp) = &c.callee {
                        if let Expr::Ident(i) = callee_exp.as_ref() {
                            if let Some(PackageMeta::Named(ExprKind::Styled)) =
                                self.import_packages.get(&i.to_id())
                            {
                                if !c.args.is_empty() {
                                    let mut args_props = Vec::with_capacity(2);
                                    args_props.push(self.create_label_prop_node("target"));
                                    self.comments.add_pure_comment(expr.span.lo());
                                    if self.options.auto_label.unwrap_or(false) {
                                        args_props.push(PropOrSpread::Prop(Box::new(
                                            Prop::KeyValue(KeyValueProp {
                                                key: PropName::Ident(Ident::new(
                                                    "label".into(),
                                                    DUMMY_SP,
                                                )),
                                                value: self.create_label(false).into(),
                                            }),
                                        )));
                                    }
                                    if let Some(cm) = self.create_sourcemap(expr.span.lo()) {
                                        expr.args.push(cm.as_arg());
                                    }
                                    if let Some(ExprOrSpread { expr, .. }) = c.args.get_mut(1) {
                                        match expr.as_mut() {
                                            Expr::Object(ObjectLit { props, .. }) => {
                                                props.extend(args_props);
                                            }
                                            Expr::Call(_) => {
                                                args_props.push(PropOrSpread::Spread(
                                                    SpreadElement {
                                                        dot3_token: DUMMY_SP,
                                                        expr: expr.take(),
                                                    },
                                                ));

                                                *expr = Box::new(Expr::Object(ObjectLit {
                                                    span: DUMMY_SP,
                                                    props: args_props,
                                                }));
                                            }
                                            _ => {
                                                c.args.push(
                                                    Expr::Object(ObjectLit {
                                                        span: DUMMY_SP,
                                                        props: args_props,
                                                    })
                                                    .as_arg(),
                                                );
                                            }
                                        }
                                    } else {
                                        c.args.push(
                                            Expr::Object(ObjectLit {
                                                span: DUMMY_SP,
                                                props: args_props,
                                            })
                                            .as_arg(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                // styled.div({})
                // customEmotionReact.css({})
                Expr::Member(m) => {
                    if let Expr::Ident(i) = m.obj.as_ref() {
                        if let Some(package) = self.import_packages.get(&i.to_id()) {
                            if let PackageMeta::Named(kind) = package {
                                if matches!(kind, ExprKind::Styled) {
                                    if let MemberProp::Ident(prop) = &m.prop {
                                        let mut args_props = Vec::with_capacity(2);
                                        args_props.push(self.create_label_prop_node("target"));
                                        let mut args = vec![prop.sym.as_ref().as_arg()];
                                        if !self.in_jsx_element {
                                            self.comments.add_pure_comment(expr.span.lo());
                                            if self.options.auto_label.unwrap_or(false) {
                                                args_props.push(PropOrSpread::Prop(Box::new(
                                                    Prop::KeyValue(KeyValueProp {
                                                        key: PropName::Ident(Ident::new(
                                                            "label".into(),
                                                            DUMMY_SP,
                                                        )),
                                                        value: self.create_label(false).into(),
                                                    }),
                                                )));
                                            }
                                            args.push(
                                                Expr::Object(ObjectLit {
                                                    span: DUMMY_SP,
                                                    props: args_props,
                                                })
                                                .as_arg(),
                                            );
                                            if let Some(cm) = self.create_sourcemap(expr.span.lo())
                                            {
                                                expr.args.push(cm.as_arg());
                                            }
                                        }
                                        return CallExpr {
                                            span: expr.span,
                                            type_args: expr.type_args,
                                            args: expr.args,
                                            callee: CallExpr {
                                                span: DUMMY_SP,
                                                type_args: None,
                                                callee: Ident::new(i.sym.clone(), i.span)
                                                    .as_callee(),
                                                args,
                                            }
                                            .as_callee(),
                                        };
                                    }
                                }
                            }
                            if let PackageMeta::Namespace(c) = package {
                                if c.exported_names
                                    .iter()
                                    .any(|n| match_css_export(n, &m.prop))
                                {
                                    self.comments.add_pure_comment(expr.span.lo());
                                    if self.options.auto_label.unwrap_or(false) {
                                        expr.args.push(self.create_label(true).as_arg());
                                    }
                                    if let Some(cm) = self.create_sourcemap(expr.span.lo()) {
                                        expr.args.push(cm.as_arg());
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        expr
    }

    fn fold_class_decl(&mut self, cd: ClassDecl) -> ClassDecl {
        self.current_class = Some(cd.ident.sym.as_ref().to_owned());
        self.current_context.clone_from(&self.current_class);
        cd.fold_children_with(self)
    }

    fn fold_class_method(&mut self, cm: ClassMethod) -> ClassMethod {
        // class methods use the class name for the context
        if self.current_class.is_some() {
            self.current_context.clone_from(&self.current_class);
        }
        cm.fold_children_with(self)
    }

    fn fold_class_prop(&mut self, cp: ClassProp) -> ClassProp {
        if let PropName::Ident(p) = &cp.key {
            self.current_context = Some(p.sym.as_ref().to_owned());
        }
        cp.fold_children_with(self)
    }

    fn fold_computed_prop_name(
        &mut self,
        n: swc_ecma_ast::ComputedPropName,
    ) -> swc_ecma_ast::ComputedPropName {
        // Existing @emotion/babel-plugin behaviour is that computed
        // properties do not have a label. We reset the label here as
        // an unset label is reduced to an empty string in `create_label`.
        self.current_context = None;
        n.fold_children_with(self)
    }

    fn fold_expr(&mut self, mut expr: Expr) -> Expr {
        if let Expr::TaggedTpl(tagged_tpl) = &mut expr {
            // styled('div')``
            match tagged_tpl.tag.as_mut() {
                Expr::Call(call) => {
                    if let Callee::Expr(callee) = &call.callee {
                        if let Expr::Ident(i) = callee.as_ref() {
                            if let Some(PackageMeta::Named(ExprKind::Styled)) =
                                self.import_packages.get(&i.to_id())
                            {
                                let mut callee = call.take();
                                let mut object_props = Vec::with_capacity(2);
                                object_props.push(self.create_label_prop_node("target"));
                                self.comments.add_pure_comment(callee.span.lo());
                                if self.options.auto_label.unwrap_or(false) {
                                    object_props.push(PropOrSpread::Prop(Box::new(
                                        Prop::KeyValue(KeyValueProp {
                                            key: PropName::Ident(Ident::new(
                                                "label".into(),
                                                DUMMY_SP,
                                            )),
                                            value: self.create_label(false).into(),
                                        }),
                                    )));
                                }
                                if let Some(ExprOrSpread { expr, .. }) = callee.args.get_mut(1) {
                                    match expr.as_mut() {
                                        Expr::Object(ObjectLit { props, .. }) => {
                                            props.extend(object_props);
                                        }
                                        Expr::Call(_) => {
                                            object_props.push(PropOrSpread::Spread(
                                                SpreadElement {
                                                    dot3_token: DUMMY_SP,
                                                    expr: expr.take(),
                                                },
                                            ));

                                            *expr = Box::new(Expr::Object(ObjectLit {
                                                span: DUMMY_SP,
                                                props: object_props,
                                            }));
                                        }
                                        _ => {
                                            callee.args.push(
                                                Expr::Object(ObjectLit {
                                                    span: DUMMY_SP,
                                                    props: object_props,
                                                })
                                                .as_arg(),
                                            );
                                        }
                                    }
                                } else {
                                    callee.args.push(
                                        Expr::Object(ObjectLit {
                                            span: DUMMY_SP,
                                            props: object_props,
                                        })
                                        .as_arg(),
                                    );
                                }
                                return Expr::Call(CallExpr {
                                    span: DUMMY_SP,
                                    callee: callee.as_callee(),
                                    args: {
                                        let mut args: Vec<ExprOrSpread> = self
                                            .create_args_from_tagged_tpl(&mut tagged_tpl.tpl)
                                            .into_iter()
                                            .map(|exp| exp.fold_children_with(self))
                                            .collect();
                                        if let Some(cm) =
                                            self.create_sourcemap(tagged_tpl.span.lo())
                                        {
                                            args.push(cm.as_arg());
                                        }
                                        args
                                    },
                                    type_args: None,
                                });
                            }
                        }
                    }
                }
                // css``
                Expr::Ident(i) => {
                    if let Some(PackageMeta::Named(ExprKind::Css)) =
                        self.import_packages.get(&i.to_id())
                    {
                        let mut args = self.create_args_from_tagged_tpl(&mut tagged_tpl.tpl);
                        if !self.in_jsx_element {
                            self.comments.add_pure_comment(i.span.lo());
                            if self.options.auto_label.unwrap_or(false) {
                                args.push(self.create_label(false).as_arg());
                            }
                            if let Some(cm) = self.create_sourcemap(tagged_tpl.span.lo()) {
                                args.push(cm.as_arg());
                            }
                        }
                        return Expr::Call(CallExpr {
                            span: DUMMY_SP,
                            callee: i.take().as_callee(),
                            args,
                            type_args: None,
                        });
                    }
                }
                // styled.div``
                // customEmotionReact.css``
                Expr::Member(member_expr) => {
                    if let Expr::Ident(i) = member_expr.obj.as_mut() {
                        if let Some(p) = self.import_packages.get(&i.to_id()) {
                            match p {
                                PackageMeta::Named(ExprKind::Styled) => {
                                    if let MemberProp::Ident(prop) = &mut member_expr.prop {
                                        let mut object_props = Vec::with_capacity(2);
                                        object_props.push(self.create_label_prop_node("target"));
                                        if self.options.auto_label.unwrap_or(false) {
                                            object_props.push(PropOrSpread::Prop(Box::new(
                                                Prop::KeyValue(KeyValueProp {
                                                    key: PropName::Ident(Ident::new(
                                                        "label".into(),
                                                        DUMMY_SP,
                                                    )),
                                                    value: self.create_label(false).into(),
                                                }),
                                            )));
                                        }
                                        let mut args =
                                            self.create_args_from_tagged_tpl(&mut tagged_tpl.tpl);

                                        if let Some(cm) =
                                            self.create_sourcemap(tagged_tpl.span.lo())
                                        {
                                            args.push(cm.as_arg());
                                        }

                                        self.comments.add_pure_comment(member_expr.span.lo());
                                        return Expr::Call(CallExpr {
                                            span: DUMMY_SP,
                                            type_args: None,
                                            callee: CallExpr {
                                                type_args: None,
                                                span: DUMMY_SP,
                                                callee: i.take().as_callee(),
                                                args: vec![
                                                    prop.take().sym.as_arg(),
                                                    Expr::Object(ObjectLit {
                                                        span: DUMMY_SP,
                                                        props: object_props,
                                                    })
                                                    .as_arg(),
                                                ],
                                            }
                                            .as_callee(),
                                            args,
                                        });
                                    }
                                }
                                PackageMeta::Namespace(c) => {
                                    if c.exported_names
                                        .iter()
                                        .any(|item| match_css_export(item, &member_expr.prop))
                                    {
                                        self.comments.add_pure_comment(member_expr.span.lo());
                                        return Expr::Call(CallExpr {
                                            span: DUMMY_SP,
                                            callee: member_expr.take().as_callee(),
                                            args: {
                                                let mut args = self.create_args_from_tagged_tpl(
                                                    &mut tagged_tpl.tpl,
                                                );
                                                if self.options.auto_label.unwrap_or(false) {
                                                    args.push(self.create_label(true).as_arg());
                                                }
                                                if let Some(cm) =
                                                    self.create_sourcemap(tagged_tpl.span.lo())
                                                {
                                                    args.push(cm.as_arg());
                                                }
                                                args
                                            },
                                            type_args: None,
                                        });
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        expr.fold_children_with(self)
    }

    fn fold_fn_decl(&mut self, fn_dec: FnDecl) -> FnDecl {
        self.current_context = Some(fn_dec.ident.sym.as_ref().to_owned());
        fn_dec.fold_children_with(self)
    }

    // Collect import modules that indicator if this file need to be transformed
    fn fold_import_decl(&mut self, expr: ImportDecl) -> ImportDecl {
        if expr.type_only {
            return expr;
        }
        self.generate_import_info(&expr);
        expr
    }

    fn fold_jsx_element(&mut self, mut expr: JSXElement) -> JSXElement {
        match &mut expr.opening.name {
            JSXElementName::Ident(i) => {
                if let Some(PackageMeta::Named(ExprKind::GlobalJSX)) =
                    self.import_packages.get(&i.to_id())
                {
                    self.rewrite_styles_attr(&mut expr.opening.attrs, i.span.lo());
                }
            }
            JSXElementName::JSXMemberExpr(member_exp) => {
                if let JSXObject::Ident(i) = &member_exp.obj {
                    if let Some(PackageMeta::Namespace(EmotionModuleConfig {
                        exported_names,
                        ..
                    })) = self.import_packages.get(&i.to_id())
                    {
                        if exported_names.iter().any(|item| {
                            matches!(item.kind, ExprKind::GlobalJSX)
                                && item.name == member_exp.prop.as_ref()
                        }) {
                            self.rewrite_styles_attr(&mut expr.opening.attrs, i.span.lo());
                        }
                    }
                }
            }
            _ => {}
        };
        let dest_expr = expr.fold_children_with(self);
        self.in_jsx_element = false;
        dest_expr
    }

    fn fold_key_value_prop(&mut self, kv: KeyValueProp) -> KeyValueProp {
        match &kv.key {
            PropName::Ident(k) => {
                self.current_context = Some(k.sym.as_ref().to_owned());
            }
            PropName::Str(k) => {
                self.current_context = Some(k.value.as_ref().to_owned());
            }
            _ => (),
        }
        kv.fold_children_with(self)
    }

    fn fold_method_prop(&mut self, mp: MethodProp) -> MethodProp {
        if let PropName::Ident(p) = &mp.key {
            self.current_context = Some(p.sym.as_ref().to_owned());
        }
        mp.fold_children_with(self)
    }

    fn fold_var_declarator(&mut self, dec: VarDeclarator) -> VarDeclarator {
        if let Pat::Ident(i) = &dec.name {
            self.current_context = Some(i.id.as_ref().to_owned());
        }

        // If we encounter a named function expression
        if let Some(Expr::Fn(f)) = dec.init.clone().map(|e| *e) {
            if let Some(i) = &f.ident {
                self.current_context = Some(i.sym.as_ref().to_owned());
            }
        }

        dec.fold_children_with(self)
    }
}

fn match_css_export(item: &ExportItem, prop: &MemberProp) -> bool {
    if matches!(item.kind, ExprKind::Css) {
        if let MemberProp::Ident(prop) = prop {
            if item.name.as_str() == prop.sym.as_ref() {
                return true;
            }
        }
    }
    false
}

#[inline]
fn minify_css_string(input: &str, is_first_item: bool, is_last_item: bool) -> Cow<str> {
    match MULTI_LINE_COMMENT.replace_all(input, "$s") {
        Cow::Borrowed(borrowed) => remove_space_and_comments(borrowed, is_first_item, is_last_item),
        Cow::Owned(owned) => remove_space_and_comments(&owned, is_first_item, is_last_item)
            .into_owned()
            .into(),
    }
}

#[inline]
fn remove_space_and_comments(input: &str, is_first_item: bool, is_last_item: bool) -> Cow<str> {
    match INVALID_SINGLE_LINE_COMMENT.replace_all(input, "$s") {
        Cow::Borrowed(borrowed) => remove_space_around_colon(borrowed, is_first_item, is_last_item),
        Cow::Owned(owned) => remove_space_around_colon(&owned, is_first_item, is_last_item)
            .into_owned()
            .into(),
    }
}

#[inline]
fn remove_space_around_colon(input: &str, is_first_item: bool, is_last_item: bool) -> Cow<str> {
    let pattern = |c| c == '\n';
    let pattern_trim_spaces = |c| c == ' ' || c == '\n';
    SPACE_AROUND_COLON.replace_all(
        input
            .trim_start_matches(if is_first_item {
                pattern_trim_spaces
            } else {
                pattern
            })
            .trim_end_matches(if is_last_item {
                pattern_trim_spaces
            } else {
                pattern
            }),
        "$s",
    )
}

#[cfg(test)]
mod test_emotion {
    use super::minify_css_string;

    #[test]
    fn should_not_trim_end_space_in_first_item() {
        assert_eq!(
            minify_css_string(
                r#"
            box-shadow: inset 0px 0px 0px "#,
                true,
                false
            ),
            "box-shadow:inset 0px 0px 0px "
        );
    }

    #[test]
    fn should_minify_single_line_comment_correctly() {
        assert_eq!(
            minify_css_string(
                "//comment;\ncolor: red;//comment\nbackground-image:url(http://dummy-url)",
                true,
                true
            ),
            "color:red;background-image:url(http://dummy-url)"
        )
    }

    #[test]
    fn should_remove_comments() {
        assert_eq!(
            minify_css_string(
                "color: red;/*comment\ncomments*/background-image:url(http://dummy-url).foo{/*comments\n*/\n}",
                true,
                true
            ),
            "color:red;background-image:url(http://dummy-url).foo{}"
        )
    }

    #[test]
    fn issue_258_should_preserve_url_starting_with_two_slashes_1() {
        assert_eq!(
            minify_css_string(
                "background-image: url('//domain.com/image.png');",
                true,
                true
            ),
            "background-image:url('//domain.com/image.png');"
        )
    }

    #[test]
    fn issue_258_should_preserve_url_starting_with_two_slashes_2() {
        assert_eq!(
            minify_css_string(
                "background-image: url(\"//domain.com/image.png\");",
                true,
                true
            ),
            "background-image:url(\"//domain.com/image.png\");"
        )
    }
}

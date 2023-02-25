#![allow(clippy::boxed_local)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use once_cell::sync::Lazy;
use swc_common::{
    comments::{Comment, CommentKind, Comments},
    util::take::Take,
    BytePos, DUMMY_SP,
};
use swc_core::{
    common::Spanned,
    ecma::{
        ast::*,
        utils::{quote_ident, ExprFactory},
        visit::{Visit, VisitMut, VisitMutWith, VisitWith},
    },
    plugin::{
        plugin_transform,
        proxies::{PluginCommentsProxy, TransformPluginProgramMetadata},
    },
    quote,
};
use tracing::debug;

use crate::util::get_import_arg;

mod util;

static JS_PATH_REGEXP: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^[./]+|(\.js$)").unwrap());

static WEBPACK_PATH_NAME_NORMALIZE_REPLACE_REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"[^a-zA-Z0-9_!§$()=\-^°]+").unwrap());

static WEBPACK_MATCH_PADDED_HYPHENS_REPLACE_REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^-|-$").unwrap());

static MATCH_LEFT_HYPHENS_REPLACE_REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^-").unwrap());

#[plugin_transform]
fn loadable_components_plugin(
    mut program: Program,
    _data: TransformPluginProgramMetadata,
) -> Program {
    program.visit_mut_with(&mut loadable_transform(PluginCommentsProxy));

    program
}

pub fn loadable_transform<C>(comments: C) -> impl VisitMut
where
    C: Comments,
{
    Loadable { comments }
}

struct Loadable<C>
where
    C: Comments,
{
    comments: C,
}

impl<C> Loadable<C>
where
    C: Comments,
{
    fn is_valid_identifier(e: &Expr) -> bool {
        match e {
            Expr::Ident(i) => &*i.sym == "loadable",
            Expr::Member(MemberExpr {
                obj,
                prop: MemberProp::Ident(prop),
                ..
            }) => match &**obj {
                Expr::Ident(i) => &*i.sym == "loadable" && &*prop.sym == "lib",
                _ => false,
            },
            _ => false,
        }
    }

    fn is_supported(&self, e: &Expr) -> bool {
        match e {
            Expr::Paren(e) => self.is_supported(&e.expr),
            Expr::Fn(..) | Expr::Arrow(..) => true,
            _ => false,
        }
    }

    fn has_loadable_comment(&self, lo: BytePos) -> bool {
        if self.comments.with_leading(lo, |comments| {
            comments
                .iter()
                .any(|comment| comment.text.contains("#__LOADABLE__"))
        }) {
            // Remove this comment
            let comments = self.comments.take_leading(lo);
            if let Some(mut comments) = comments {
                comments.retain(|c| !c.text.contains("#__LOADABLE__"));
                self.comments.add_leading_comments(lo, comments)
            }
            return true;
        }

        false
    }

    fn transform_import_expr(&mut self, call: &mut CallExpr) {
        let import = {
            let mut v = ImportFinder::default();
            call.visit_with(&mut v);
            match v.res {
                Some(v) => v,
                None => return,
            }
        };

        match call.args.get(0) {
            Some(arg) if self.is_supported(&arg.expr) => {}
            _ => return,
        }

        let object = self.create_object_from(&import, &call.args[0].expr);
        call.args[0] = object.as_arg();
    }

    fn create_object_from(&mut self, import: &CallExpr, func: &Expr) -> Expr {
        ObjectLit {
            span: DUMMY_SP,
            props: vec![
                PropOrSpread::Prop(Box::new(Prop::KeyValue(
                    self.create_resolved_property(import, func),
                ))),
                PropOrSpread::Prop(Box::new(Prop::Method(
                    self.create_chunk_name_method(import, func),
                ))),
                PropOrSpread::Prop(Box::new(Prop::Method(
                    self.create_is_ready_method(import, func),
                ))),
                PropOrSpread::Prop(Box::new(Prop::KeyValue(
                    self.create_import_async_property(import, func),
                ))),
                PropOrSpread::Prop(Box::new(Prop::Method(
                    self.create_require_async_method(import, func),
                ))),
                PropOrSpread::Prop(Box::new(Prop::Method(
                    self.create_require_sync_method(import, func),
                ))),
                PropOrSpread::Prop(Box::new(Prop::Method(
                    self.create_resolve_method(import, func),
                ))),
            ],
        }
        .into()
    }

    fn create_resolved_property(&mut self, _import: &CallExpr, _func: &Expr) -> KeyValueProp {
        KeyValueProp {
            key: PropName::Ident(quote_ident!("resolved")),
            value: Box::new(
                ObjectLit {
                    span: DUMMY_SP,
                    props: Default::default(),
                }
                .into(),
            ),
        }
    }

    fn is_aggressive_import(&self, import: &CallExpr) -> bool {
        let import_arg = get_import_arg(import);
        match import_arg {
            Expr::Tpl(t) => !t.exprs.is_empty(),
            _ => false,
        }
    }

    fn get_chunk_name_content(&self, import_arg: &Expr) -> Option<String> {
        if !self.comments.has_leading(import_arg.span_lo()) {
            return None;
        }

        self.comments
            .with_leading(import_arg.span_lo(), |comments| {
                comments
                    .iter()
                    .find(|c| c.text.contains("webpackChunkName"))
                    .map(|v| v.text.to_string())
            })
    }

    fn read_webpack_comment_values(&self, v: String) -> serde_json::Value {
        serde_json::Value::Object(
            v.split(',')
                .map(|v| v.trim())
                .filter_map(|item| {
                    let s = item.split(':').map(|s| s.trim()).collect::<Vec<_>>();
                    if s.len() == 2 {
                        return Some((
                            s[0].to_string(),
                            serde_json::Value::String(s[1].trim_matches('"').to_string()),
                        ));
                    }

                    None
                })
                .collect::<serde_json::Map<_, _>>(),
        )
    }

    fn get_raw_chunk_name_from_comments(&self, import_arg: &Expr) -> Option<serde_json::Value> {
        let chunk_name_comment = self.get_chunk_name_content(import_arg);

        chunk_name_comment.map(|v| self.read_webpack_comment_values(v))
    }

    fn get_existing_chunk_name_comment(&self, import: &CallExpr) -> Option<serde_json::Value> {
        let import_arg = get_import_arg(import);

        self.get_raw_chunk_name_from_comments(import_arg)
    }

    fn chunk_name_from_template_literal(&self, node: &Expr) -> String {
        match node {
            Expr::Tpl(t) => {
                let v1 = t.quasis[0].cooked.clone().unwrap_or_default();
                if t.exprs.is_empty() {
                    return v1.to_string();
                }

                format!("{}[request]", v1)
            }
            _ => unreachable!(),
        }
    }

    fn add_or_replace_chunk_name_comment(&self, import: &CallExpr, values: serde_json::Value) {
        let import_arg = get_import_arg(import);

        let chunk_name_content = self.get_chunk_name_content(import_arg);
        if chunk_name_content.is_some() {
            let comments = self.comments.take_leading(import_arg.span_lo());

            if let Some(mut comments) = comments {
                comments.retain(|c| !c.text.contains("webpackChunkName"));
                self.comments
                    .add_leading_comments(import_arg.span_lo(), comments)
            }
        }

        self.comments.add_leading(
            import_arg.span_lo(),
            Comment {
                kind: CommentKind::Block,
                span: DUMMY_SP,
                text: self.write_webpack_comment_values(values).into(),
            },
        )
    }

    fn replace_chunk_name(&self, import: &CallExpr) -> Expr {
        let aggressive_import = self.is_aggressive_import(import);
        let values = self.get_existing_chunk_name_comment(import);

        debug!("Values: {:#?}", values);

        let mut webpack_chunk_name = values
            .as_ref()
            .map(|map| map["webpackChunkName"].as_str().map(|v| v.to_string()))
            .unwrap_or_default();

        if !aggressive_import {
            if let Some(values) = values {
                self.add_or_replace_chunk_name_comment(import, values);
                return webpack_chunk_name.unwrap().into();
            }
        }

        let mut chunk_name_node = self.generate_chunk_name_node(
            import,
            self.get_chunk_name_prefix(webpack_chunk_name.as_deref()),
        );

        if chunk_name_node.is_tpl() {
            webpack_chunk_name = Some(self.chunk_name_from_template_literal(&chunk_name_node));
            chunk_name_node = self.sanitize_chunk_name_template_literal(Box::new(chunk_name_node));
        } else if let Expr::Lit(Lit::Str(s)) = &chunk_name_node {
            webpack_chunk_name = Some(s.value.to_string());
        }
        let mut values = values.unwrap_or_default();

        if let Some(webpack_chunk_name) = webpack_chunk_name {
            values["webpackChunkName"] = serde_json::Value::String(webpack_chunk_name);
        } else {
            values["webpackChunkName"] = serde_json::Value::Null;
        }
        self.add_or_replace_chunk_name_comment(import, values);
        chunk_name_node
    }

    fn create_chunk_name_method(&mut self, import: &CallExpr, func: &Expr) -> MethodProp {
        MethodProp {
            key: PropName::Ident(quote_ident!("chunkName")),
            function: Box::new(Function {
                params: clone_params(func),
                decorators: Default::default(),
                span: DUMMY_SP,
                body: Some(BlockStmt {
                    span: DUMMY_SP,
                    stmts: vec![Stmt::Return(ReturnStmt {
                        span: DUMMY_SP,
                        arg: Some(Box::new(self.replace_chunk_name(import))),
                    })],
                }),
                is_generator: false,
                is_async: false,
                type_params: Default::default(),
                return_type: Default::default(),
            }),
        }
    }

    fn create_is_ready_method(&mut self, _import: &CallExpr, _func: &Expr) -> MethodProp {
        MethodProp {
            key: PropName::Ident(quote_ident!("isReady")),
            function: Box::new(Function {
                params: vec![Param {
                    span: DUMMY_SP,
                    decorators: Default::default(),
                    pat: Pat::Ident(quote_ident!("props").into()),
                }],
                decorators: Default::default(),
                span: DUMMY_SP,
                body: Some(
                    quote!(
                        "
                        {
                            const key=this.resolve(props)
                            if (this.resolved[key] !== true) {
                                return false
                            }

                            if (typeof __webpack_modules__ !== 'undefined') {
                                return !!(__webpack_modules__[key])
                            }

                            return false
                        }
                      " as Stmt
                    )
                    .expect_block(),
                ),
                is_generator: false,
                is_async: false,
                type_params: Default::default(),
                return_type: Default::default(),
            }),
        }
    }

    fn create_import_async_property(&mut self, _import: &CallExpr, func: &Expr) -> KeyValueProp {
        KeyValueProp {
            key: PropName::Ident(quote_ident!("importAsync")),
            value: Box::new(func.clone()),
        }
    }

    fn create_require_async_method(&mut self, _import: &CallExpr, _func: &Expr) -> MethodProp {
        MethodProp {
            key: PropName::Ident(quote_ident!("requireAsync")),
            function: Box::new(Function {
                params: vec![Param {
                    span: DUMMY_SP,
                    decorators: Default::default(),
                    pat: Pat::Ident(quote_ident!("props").into()),
                }],
                decorators: Default::default(),
                span: DUMMY_SP,
                body: Some(
                    quote!(
                        "
                        {
                            const key = this.resolve(props)
                            this.resolved[key] = false
                            return this.importAsync(props).then(resolved => {
                                this.resolved[key] = true
                                return resolved;
                            });
                        }
                        " as Stmt
                    )
                    .expect_block(),
                ),
                is_generator: false,
                is_async: false,
                type_params: Default::default(),
                return_type: Default::default(),
            }),
        }
    }

    fn create_require_sync_method(&mut self, _import: &CallExpr, _func: &Expr) -> MethodProp {
        MethodProp {
            key: PropName::Ident(quote_ident!("requireSync")),
            function: Box::new(Function {
                params: vec![Param {
                    span: DUMMY_SP,
                    decorators: Default::default(),
                    pat: Pat::Ident(quote_ident!("props").into()),
                }],
                decorators: Default::default(),
                span: DUMMY_SP,
                body: Some(
                    quote!(
                        "
                    {
                        const id = this.resolve(props)

                        if (typeof __webpack_require__ !== 'undefined') {
                        return __webpack_require__(id)
                        }

                        return eval('module.require')(id)
                    }
                    " as Stmt
                    )
                    .expect_block(),
                ),
                is_generator: false,
                is_async: false,
                type_params: Default::default(),
                return_type: Default::default(),
            }),
        }
    }

    fn create_resolve_method(&mut self, import: &CallExpr, func: &Expr) -> MethodProp {
        fn get_call_value(import: &CallExpr) -> Expr {
            let import_arg = get_import_arg(import);

            import_arg.clone()
        }

        MethodProp {
            key: PropName::Ident(quote_ident!("resolve")),
            function: Box::new(Function {
                params: clone_params(func),
                decorators: Default::default(),
                span: DUMMY_SP,
                body: Some(
                    quote!(
                        "
                        {
                            if (require.resolveWeak) {
                                return require.resolveWeak($id)
                              }
                          
                              return eval('require.resolve')($id)
                        }
                        " as Stmt,
                        id: Expr = get_call_value(import)
                    )
                    .expect_block(),
                ),
                is_generator: false,
                is_async: false,
                type_params: Default::default(),
                return_type: Default::default(),
            }),
        }
    }

    fn write_webpack_comment_values(&self, values: serde_json::Value) -> String {
        values
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn get_chunk_name_prefix(&self, chunk_name: Option<&str>) -> Option<String> {
        let chunk_name = match chunk_name {
            Some(s) => s,
            _ => return Default::default(),
        };

        if let Some(idx) = chunk_name.find("[request]") {
            return Some(chunk_name[..idx].into());
        }

        if let Some(idx) = chunk_name.find("[index]") {
            return Some(chunk_name[..idx].into());
        }

        Default::default()
    }

    fn generate_chunk_name_node(&self, import: &CallExpr, prefix: Option<String>) -> Expr {
        let import_arg = get_import_arg(import);

        if let Expr::Tpl(import_arg) = import_arg {
            return prefix
                .map(|prefix| {
                    prefix.make_bin(
                        op!(bin, "+"),
                        self.sanitize_chunk_name_template_literal(
                            self.combine_expression(import_arg),
                        ),
                    )
                })
                .unwrap_or_else(|| {
                    Expr::Tpl(Tpl {
                        span: DUMMY_SP,
                        exprs: import_arg.exprs.clone(),
                        quasis: import_arg
                            .quasis
                            .iter()
                            .enumerate()
                            .map(|(idx, quasi)| {
                                self.transform_quasi(quasi, idx == 0, import_arg.quasis.len() == 1)
                            })
                            .collect(),
                    })
                });
        }

        let value = match import_arg {
            Expr::Lit(Lit::Str(s)) => s.value.clone(),
            _ => return "".into(),
        };
        self.module_to_chunk(&value).into()
    }

    fn sanitize_chunk_name_template_literal(&self, node: Box<Expr>) -> Expr {
        Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: node.make_member(quote_ident!("replace")).as_callee(),
            args: vec![
                Lit::Regex(Regex {
                    span: DUMMY_SP,
                    exp: "[^a-zA-Z0-9_!§$()=\\\\-^°]+".into(),
                    flags: "g".into(),
                })
                .as_arg(),
                "-".as_arg(),
            ],
            type_args: Default::default(),
        })
    }

    fn transform_quasi(&self, quasi: &TplElement, first: bool, single: bool) -> TplElement {
        TplElement {
            span: quasi.span,
            tail: quasi.tail,
            cooked: quasi.cooked.as_ref().map(|cooked| {
                if single {
                    self.module_to_chunk(cooked).into()
                } else {
                    self.replace_quasi(cooked, first).into()
                }
            }),
            raw: if single {
                self.module_to_chunk(&quasi.raw).into()
            } else {
                self.replace_quasi(&quasi.raw, first).into()
            },
        }
    }

    fn replace_quasi(&self, s: &str, strip_left_hyphen: bool) -> String {
        debug!("replace_quasi: `{}`", s);

        if s.is_empty() {
            return Default::default();
        }
        let s = WEBPACK_PATH_NAME_NORMALIZE_REPLACE_REGEX.replace_all(s, "-");

        if strip_left_hyphen {
            let s = MATCH_LEFT_HYPHENS_REPLACE_REGEX.replace_all(&s, "");

            debug!("replace_quasi: result: `{}`", s);

            s.into()
        } else {
            debug!("replace_quasi: result: `{}`", s);

            s.into()
        }
    }

    fn module_to_chunk(&self, s: &str) -> String {
        debug!("module_to_chunk: `{}`", s);

        let s = JS_PATH_REGEXP.replace_all(s, "");
        let s = WEBPACK_PATH_NAME_NORMALIZE_REPLACE_REGEX.replace_all(&s, "-");
        let s = WEBPACK_MATCH_PADDED_HYPHENS_REPLACE_REGEX.replace_all(&s, "");

        debug!("module_to_chunk: result: `{}`", s);

        s.into_owned()
    }

    fn combine_expression(&self, node: &Tpl) -> Box<Expr> {
        if node.exprs.len() == 1 {
            return node.exprs[0].clone();
        }

        node.exprs
            .iter()
            .skip(1)
            .cloned()
            .fold(node.exprs[0].clone(), |r, p| {
                Box::new(r.make_bin(op!(bin, "+"), *p))
            })
    }
}

impl<C> VisitMut for Loadable<C>
where
    C: Comments,
{
    fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
        call.visit_mut_children_with(self);

        match &call.callee {
            Callee::Expr(callee) if Self::is_valid_identifier(callee) => {}
            _ => return,
        }

        // Transform imports
        self.transform_import_expr(call)
    }

    fn visit_mut_expr(&mut self, e: &mut Expr) {
        e.visit_mut_children_with(self);

        match e {
            Expr::Arrow(..) | Expr::Fn(..) => {
                if !self.has_loadable_comment(e.span_lo()) {
                    return;
                }

                let import = {
                    let mut v = ImportFinder::default();
                    e.visit_with(&mut v);
                    match v.res {
                        Some(v) => v,
                        None => return,
                    }
                };

                let object = self.create_object_from(&import, e);
                *e = object;
            }
            _ => {}
        }
    }

    fn visit_mut_prop(&mut self, n: &mut Prop) {
        n.visit_mut_children_with(self);

        if let Prop::Method(m) = n {
            if !self.has_loadable_comment(m.span_lo()) {
                return;
            }

            let import = {
                let mut v = ImportFinder::default();
                m.visit_with(&mut v);
                match v.res {
                    Some(v) => v,
                    None => return,
                }
            };

            let object = self.create_object_from(
                &import,
                &Expr::Fn(FnExpr {
                    ident: None,
                    function: m.function.take(),
                }),
            );
            *n = Prop::KeyValue(KeyValueProp {
                key: m.key.take(),
                value: Box::new(object),
            });
        }
    }
}

#[derive(Default)]
struct ImportFinder {
    res: Option<CallExpr>,
}

impl Visit for ImportFinder {
    fn visit_call_expr(&mut self, call: &CallExpr) {
        match &call.callee {
            Callee::Import(..) => {
                if self.res.is_some() {
                    panic!(
                        "loadable: multiple import calls inside `loadable()` function are not \
                         supported."
                    );
                }
                self.res = Some(call.clone());
            }
            _ => {
                call.visit_children_with(self);
            }
        }
    }
}

fn clone_params(e: &Expr) -> Vec<Param> {
    match e {
        Expr::Fn(f) => f.function.params.clone(),
        Expr::Arrow(f) => f
            .params
            .iter()
            .cloned()
            .map(|pat| Param {
                span: DUMMY_SP,
                pat,
                decorators: Default::default(),
            })
            .collect(),
        _ => Default::default(),
    }
}

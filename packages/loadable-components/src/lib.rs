use std::fmt::format;

use swc_common::{comments::Comments, DUMMY_SP};
use swc_core::{
    ast::*,
    common::Spanned,
    plugin::{
        plugin_transform,
        proxies::{PluginCommentsProxy, TransformPluginProgramMetadata},
    },
    quote,
    utils::{quote_ident, ExprFactory},
    visit::{Visit, VisitMut, VisitMutWith, VisitWith},
};

use crate::util::get_import_arg;

mod util;

#[plugin_transform]
fn loadable_components_plugin(
    mut program: Program,
    data: TransformPluginProgramMetadata,
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
            Expr::Call(CallExpr {
                callee: Callee::Expr(callee),
                ..
            }) => match &**callee {
                Expr::Member(MemberExpr {
                    obj,
                    prop: MemberProp::Ident(prop),
                    ..
                }) => match &**obj {
                    Expr::Ident(i) => &*i.sym == "loadable" && &*prop.sym == "lib",
                    _ => false,
                },
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

    fn transform_import(&mut self, call: &mut CallExpr) {
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
                    .find(|c| c.text.contains("webpackChunkName   "))
            })
            .map(|v| v.text.to_string())
    }

    fn read_webpack_comment_values(&self, v: String) -> String {
        // TODO?
        v
    }

    fn get_raw_chunk_name_from_comments(&self, import_arg: &Expr) -> Option<serde_json::Value> {
        let chunk_name_comment = self.get_chunk_name_content(import_arg);

        chunk_name_comment
            .map(|v| self.read_webpack_comment_values(v))
            .map(From::from)
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
            // TODO: Restore unrelated comments
            let comments = self.comments.take_leading(import_arg.span_lo());
        }

        self.comments.add_leading(
            import_arg.span_lo(),
            self.write_webpack_comment_values(values),
        )
    }

    fn replace_chunk_name(&self, import: &CallExpr) -> Expr {
        let aggressive_import = self.is_aggressive_import(import);
        let mut values = self.get_existing_chunk_name_comment(import);
        let mut webpack_chunk_name = values.unwrap_or_default()["webpackChunkName"]
            .as_str()
            .map(|v| v.to_string());

        if aggressive_import && values.is_some() {
            self.add_or_replace_chunk_name_comment(import, values.unwrap());
            return webpack_chunk_name.unwrap().into();
        }

        let mut chunk_name_node =
            self.generateChunkNameNode(import, self.getChunkNamePrefix(values));

        if chunk_name_node.is_tpl() {
            webpack_chunk_name = Some(self.chunk_name_from_template_literal(chunk_name_node));
            chunk_name_node = self.sanitizeChunkNameTemplateLiteral(chunk_name_node);
        } else if let Expr::Lit(Lit::Str(s)) = &chunk_name_node {
            webpack_chunk_name = Some(s.value.to_string());
        }
        let mut values = values.unwrap_or_default();

        values["webpackChunkName"] = webpack_chunk_name;
        self.add_or_replace_chunk_name_comment(import, values);
        chunk_name_node
    }

    fn create_chunk_name_method(&mut self, import: &CallExpr, func: &Expr) -> MethodProp {
        MethodProp {
            key: PropName::Ident(quote_ident!("chunkName")),
            function: Function {
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
            },
        }
    }

    fn create_is_ready_method(&mut self, import: &CallExpr, func: &Expr) -> MethodProp {
        MethodProp {
            key: PropName::Ident(quote_ident!("isReady")),
            function: Function {
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
            },
        }
    }

    fn create_import_async_property(&mut self, _import: &CallExpr, func: &Expr) -> KeyValueProp {
        KeyValueProp {
            key: PropName::Ident(quote_ident!("importAsync")),
            value: Box::new(func.clone()),
        }
    }

    fn create_require_async_method(&mut self, import: &CallExpr, func: &Expr) -> MethodProp {
        MethodProp {
            key: PropName::Ident(quote_ident!("requireAsync")),
            function: Function {
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
            },
        }
    }

    fn create_require_sync_method(&mut self, _import: &CallExpr, _func: &Expr) -> MethodProp {
        MethodProp {
            key: PropName::Ident(quote_ident!("requireSync")),
            function: Function {
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
            },
        }
    }

    fn create_resolve_method(&mut self, import: &CallExpr, func: &Expr) -> MethodProp {
        fn get_call_value(import: &CallExpr) -> Expr {
            let import_arg = get_import_arg(import);

            import_arg.clone()
        }

        MethodProp {
            key: PropName::Ident(quote_ident!("resolve")),
            function: Function {
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
            },
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
}

impl<C> VisitMut for Loadable<C>
where
    C: Comments,
{
    fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
        match &call.callee {
            Callee::Expr(callee) if Self::is_valid_identifier(callee) => {}
            _ => {
                call.visit_mut_children_with(self);
                return;
            }
        }

        // Transform imports
        self.transform_import(call)
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

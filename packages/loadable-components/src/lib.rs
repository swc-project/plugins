use swc_common::{comments::Comments, DUMMY_SP};
use swc_core::{
    ast::*,
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

    fn create_chunk_name_method(&mut self, import: &CallExpr, func: &Expr) -> MethodProp {
        MethodProp {
            key: PropName::Ident(quote_ident!("chunkName")),
            function: Function {
                params: Default::default(),
                decorators: Default::default(),
                span: DUMMY_SP,
                body: Some(BlockStmt {
                    span: DUMMY_SP,
                    stmts: vec![],
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

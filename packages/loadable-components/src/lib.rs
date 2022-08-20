use swc_common::DUMMY_SP;
use swc_core::{
    ast::*,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
    utils::{quote_ident, ExprFactory},
    visit::{Visit, VisitMut, VisitMutWith, VisitWith},
};

#[plugin_transform]
fn loadable_components_plugin(
    mut program: Program,
    data: TransformPluginProgramMetadata,
) -> Program {
    program.visit_mut_with(&mut loadable_transform());

    program
}

pub fn loadable_transform() -> impl VisitMut {
    Loadable {}
}

struct Loadable {}

impl Loadable {
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

    fn create_resolved_property(&mut self, import: &CallExpr, func: &Expr) -> KeyValueProp {
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

    fn create_import_async_property(&mut self, import: &CallExpr, func: &Expr) -> KeyValueProp {
        KeyValueProp {
            key: PropName::Ident(quote_ident!("importAsync")),
            value: Box::new(
                ObjectLit {
                    span: DUMMY_SP,
                    props: Default::default(),
                }
                .into(),
            ),
        }
    }

    fn create_require_async_method(&mut self, import: &CallExpr, func: &Expr) -> MethodProp {
        MethodProp {
            key: PropName::Ident(quote_ident!("requiredAsync")),
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

    fn create_require_sync_method(&mut self, import: &CallExpr, func: &Expr) -> MethodProp {
        MethodProp {
            key: PropName::Ident(quote_ident!("requireSync")),
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

    fn create_resolve_method(&mut self, import: &CallExpr, func: &Expr) -> MethodProp {
        MethodProp {
            key: PropName::Ident(quote_ident!("resolve")),
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
}

impl VisitMut for Loadable {
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

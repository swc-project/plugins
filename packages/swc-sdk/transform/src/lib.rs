#![feature(box_patterns)]
#![feature(never_type)]

use swc_common::{comments::Comments, errors::HANDLER, util::take::Take, Mark, DUMMY_SP};
use swc_ecma_ast::{
    ArrowExpr, AwaitExpr, BlockStmtOrExpr, CallExpr, Callee, Decl, Expr, Function, Ident,
    IdentName, Import, JSXElementName, MemberExpr, MemberProp, Module, ModuleDecl, ModuleItem,
    Stmt, VarDecl, VarDeclKind, VarDeclarator,
};
use swc_ecma_utils::{private_ident, ExprFactory};
use swc_ecma_visit::{VisitMut, VisitMutWith};

use crate::{config::Config, import_analyzer::ImportMap};

pub mod config;
mod flag;
mod import_analyzer;

#[derive(Debug, Clone)]
pub struct Env {
    pub unresolved_mark: Mark,
}

pub fn swc_sdk<C>(env: Env, config: Config, comments: C) -> impl VisitMut
where
    C: Comments,
{
    SwcSdkTransform {
        env,
        config,
        comments,
        imports: Default::default(),
        can_use_await: Default::default(),
        jsx_lazy_import: Default::default(),
        vars: Default::default(),
    }
}

/// Handles functions from `@swc/sdk`.
struct SwcSdkTransform<C>
where
    C: Comments,
{
    #[allow(unused)]
    env: Env,
    config: Config,
    #[allow(unused)]
    comments: C,
    imports: ImportMap,

    jsx_lazy_import: Option<Ident>,

    can_use_await: bool,

    vars: Vec<VarDeclarator>,
}

impl<C> SwcSdkTransform<C>
where
    C: Comments,
{
    fn replace_ident_with_dynamic_import(&mut self, ident: &Ident) -> Option<Expr> {
        let node = Expr::Ident(ident.clone());

        if let Some((import_span, source, export_name)) = self.imports.should_make_dynamic(&node) {
            if self.can_use_await {
                let import = Expr::Await(AwaitExpr {
                    span: DUMMY_SP,
                    arg: CallExpr {
                        span: import_span,
                        callee: Callee::Import(Import {
                            span: import_span,
                            phase: Default::default(),
                        }),
                        args: vec![source.clone().as_arg()],
                        ..Default::default()
                    }
                    .into(),
                });

                let member_expr = MemberExpr {
                    span: DUMMY_SP,
                    obj: import.into(),
                    prop: MemberProp::Ident(IdentName::new(export_name.clone(), ident.span)),
                };

                return Some(Expr::Member(member_expr));
            } else {
                HANDLER.with(|handler| {
                    handler
                        .struct_span_err(ident.span, "await is not allowed in this context.")
                        .help("/*#__DYNAMIC__#*/ requires async context for dynamic imports")
                        .emit();
                });
            }
        }

        None
    }
}

impl<C> VisitMut for SwcSdkTransform<C>
where
    C: Comments,
{
    fn visit_mut_arrow_expr(&mut self, node: &mut ArrowExpr) {
        let old_can_use_await = self.can_use_await;
        self.can_use_await = node.is_async;
        node.visit_mut_children_with(self);

        self.can_use_await = old_can_use_await;
    }

    fn visit_mut_expr(&mut self, node: &mut Expr) {
        if let Expr::JSXElement(jsx) = &mut *node {
            if let JSXElementName::Ident(ident) = &jsx.opening.name {
                let node = Expr::Ident(ident.clone());

                if let Some((import_span, source, export_name)) =
                    self.imports.should_make_dynamic(&node)
                {
                    let module_param = private_ident!("module");

                    let import_call = CallExpr {
                        span: import_span,
                        callee: Callee::Import(Import {
                            span: import_span,
                            phase: Default::default(),
                        }),
                        args: vec![source.clone().as_arg()],
                        ..Default::default()
                    }
                    .into();

                    let then_arg = Expr::Arrow(ArrowExpr {
                        span: DUMMY_SP,
                        params: vec![module_param.clone().into()],
                        body: BlockStmtOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
                            span: DUMMY_SP,
                            obj: module_param.clone().into(),
                            prop: MemberProp::Ident(IdentName::new(export_name.clone(), DUMMY_SP)),
                        })))
                        .into(),
                        is_async: true,
                        is_generator: false,
                        ..Default::default()
                    });

                    let then_expr = Expr::Call(CallExpr {
                        span: DUMMY_SP,
                        callee: MemberExpr {
                            span: DUMMY_SP,
                            obj: import_call,
                            prop: MemberProp::Ident("then".into()),
                        }
                        .as_callee(),
                        args: vec![then_arg.as_arg()],
                        ..Default::default()
                    });

                    let lazy_import = self
                        .jsx_lazy_import
                        .get_or_insert_with(|| private_ident!("lazy"));

                    let lazy_component = CallExpr {
                        span: DUMMY_SP,
                        callee: lazy_import.clone().as_callee(),
                        args: vec![then_expr.as_arg()],
                        ..Default::default()
                    };

                    let lazy_var_name = private_ident!("LazyComponent");

                    self.vars.push(VarDeclarator {
                        span: DUMMY_SP,
                        name: lazy_var_name.clone().into(),
                        init: Some(lazy_component.into()),
                        definite: false,
                    });

                    jsx.opening.name = JSXElementName::Ident(lazy_var_name.clone());
                    return;
                }
            }
        } else if let Expr::Call(call) = &mut *node {
            if let Callee::Expr(callee) = &mut call.callee {
                if let Some((import_span, source, export_name)) =
                    self.imports.should_make_dynamic(callee)
                {
                    let module_param = private_ident!("module");

                    let import_call = CallExpr {
                        span: import_span,
                        callee: Callee::Import(Import {
                            span: import_span,
                            phase: Default::default(),
                        }),
                        args: vec![source.clone().as_arg()],
                        ..Default::default()
                    }
                    .into();

                    let then_arg = Expr::Arrow(ArrowExpr {
                        span: DUMMY_SP,
                        params: vec![module_param.clone().into()],
                        body: Box::new(BlockStmtOrExpr::Expr(
                            CallExpr {
                                span: call.span,
                                ctxt: call.ctxt,
                                callee: MemberExpr {
                                    span: DUMMY_SP,
                                    obj: module_param.clone().into(),
                                    prop: MemberProp::Ident(IdentName::new(
                                        export_name.clone(),
                                        DUMMY_SP,
                                    )),
                                }
                                .as_callee(),
                                args: call.args.take(),
                                ..Default::default()
                            }
                            .into(),
                        )),
                        is_async: true,
                        is_generator: false,
                        ..Default::default()
                    });

                    let then_expr = Expr::Call(CallExpr {
                        span: DUMMY_SP,
                        callee: MemberExpr {
                            span: DUMMY_SP,
                            obj: import_call,
                            prop: MemberProp::Ident("then".into()),
                        }
                        .as_callee(),
                        args: vec![then_arg.as_arg()],
                        ..Default::default()
                    });

                    *node = then_expr;
                    return;
                }
            }
        }

        node.visit_mut_children_with(self);

        if let Expr::Ident(ident) = &*node {
            if let Some(expr) = self.replace_ident_with_dynamic_import(ident) {
                *node = expr;
            }
        }
    }

    fn visit_mut_function(&mut self, node: &mut Function) {
        let old_can_use_await = self.can_use_await;
        self.can_use_await = node.is_async;
        node.visit_mut_children_with(self);

        self.can_use_await = old_can_use_await;
    }

    fn visit_mut_module(&mut self, m: &mut Module) {
        self.imports = ImportMap::analyze(m, &self.comments);

        m.visit_mut_children_with(self);
    }

    fn visit_mut_module_item(&mut self, m: &mut ModuleItem) {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = m {
            if self.config.remove_imports_from.contains(&import.src.value) {
                m.take();
                return;
            }
        }

        m.visit_mut_children_with(self);
    }

    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        let mut new_stmts = Vec::with_capacity(stmts.len());
        let old_vars = self.vars.take();

        for mut stmt in stmts.take() {
            stmt.visit_mut_with(self);

            if !self.vars.is_empty() {
                stmt = ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
                    span: DUMMY_SP,
                    ctxt: Default::default(),
                    kind: VarDeclKind::Const,
                    declare: false,
                    decls: self.vars.take(),
                }))));
            }

            new_stmts.push(stmt);
        }

        *stmts = new_stmts;
        self.vars = old_vars;
    }

    fn visit_mut_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        let mut new_stmts = Vec::with_capacity(stmts.len());
        let old_vars = self.vars.take();

        for mut stmt in stmts.take() {
            stmt.visit_mut_with(self);

            if !self.vars.is_empty() {
                stmts.push(Stmt::Decl(Decl::Var(Box::new(VarDecl {
                    span: DUMMY_SP,
                    ctxt: Default::default(),
                    kind: VarDeclKind::Const,
                    declare: false,
                    decls: self.vars.take(),
                }))));
            }

            new_stmts.push(stmt);
        }

        *stmts = new_stmts;
        self.vars = old_vars;
    }

    fn visit_mut_var_declarator(&mut self, node: &mut VarDeclarator) {
        self.transform_flag(node);

        node.visit_mut_children_with(self);
    }
}

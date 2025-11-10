#![feature(box_patterns)]
#![feature(never_type)]

use swc_common::{comments::Comments, errors::HANDLER, util::take::Take, Mark, DUMMY_SP};
use swc_ecma_ast::{
    ArrowExpr, AwaitExpr, CallExpr, Callee, Expr, Function, IdentName, Import, MemberExpr,
    MemberProp, Module, ModuleDecl, ModuleItem, VarDeclarator,
};
use swc_ecma_utils::ExprFactory;
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

    can_use_await: bool,
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
        node.visit_mut_children_with(self);

        if let Expr::Ident(ident) = &*node {
            if let Some((import_span, source, export_name)) = self.imports.should_make_dynamic(node)
            {
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

                    *node = Expr::Member(member_expr);
                } else {
                    HANDLER.with(|handler| {
                        handler
                            .struct_span_err(ident.span, "await is not allowed in this context")
                            .emit();
                    });
                }
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

    fn visit_mut_var_declarator(&mut self, node: &mut VarDeclarator) {
        self.transform_flag(node);

        node.visit_mut_children_with(self);
    }
}

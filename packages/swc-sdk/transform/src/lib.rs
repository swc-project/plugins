#![feature(box_patterns)]

use swc_common::{
    comments::Comments, errors::HANDLER, util::take::Take, Mark, Span, Spanned, DUMMY_SP,
};
use swc_ecma_ast::{CallExpr, Callee, EmptyStmt, Expr, Module, ModuleDecl, ModuleItem, Stmt};
use swc_ecma_visit::{VisitMut, VisitMutWith};

use crate::{config::Config, import_analyzer::ImportMap};

pub mod config;
mod import_analyzer;

#[derive(Debug, Clone)]
pub struct Env {
    pub unresolved_mark: Mark,
    pub top_level_mark: Mark,
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
    }
}

const MARK_AS_PURE_FN_NAME: &str = "markAsPure";

/// Handles functions from `@swc/sdk`.
struct SwcSdkTransform<C>
where
    C: Comments,
{
    env: Env,
    config: Config,
    comments: C,
    imports: ImportMap,
}

impl<C> VisitMut for SwcSdkTransform<C>
where
    C: Comments,
{
    fn visit_mut_expr(&mut self, e: &mut Expr) {
        e.visit_mut_children_with(self);

        if let Expr::Call(CallExpr {
            span,
            callee: Callee::Expr(callee),
            args,
            ..
        }) = e
        {
            if !self
                .imports
                .is_import(callee, &self.config.import_path, MARK_AS_PURE_FN_NAME)
            {
                return;
            }

            if args.len() != 1 {
                HANDLER.with(|handler| {
                    handler
                        .struct_span_err(*span, "markAsPure() does not support multiple arguments")
                        .emit();
                });
                return;
            }

            *e = *args[0].expr.take();

            let mut lo = e.span().lo;
            if lo.is_dummy() {
                lo = Span::dummy_with_cmt().lo;
            }

            self.comments.add_pure_comment(lo);
        }
    }

    fn visit_mut_module(&mut self, m: &mut Module) {
        self.imports = ImportMap::analyze(m);

        m.visit_mut_children_with(self);
    }

    fn visit_mut_module_item(&mut self, m: &mut ModuleItem) {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = m {
            if import.src.value == self.config.import_path {
                *m = ModuleItem::Stmt(Stmt::Empty(EmptyStmt { span: DUMMY_SP }));
                return;
            }
        }

        m.visit_mut_children_with(self);
    }
}

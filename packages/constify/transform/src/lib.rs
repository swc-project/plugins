#![feature(box_patterns)]

use import_analyzer::ImportMap;
use rustc_hash::FxHashSet;
use swc_core::{
    common::{sync::Lazy, util::take::Take, Mark, Span, Spanned, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            CallExpr, Callee, Decl, Expr, Id, Ident, ImportDecl, Module, ModuleItem, Stmt, VarDecl,
            VarDeclKind, VarDeclarator,
        },
        atoms::JsWord,
        utils::{private_ident, StmtLike},
        visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
    },
};

use crate::utils::ids_used_by;

pub fn constify() -> impl VisitMut {
    Constify {
        const_ctxt: SyntaxContext::empty().apply_mark(Mark::new()),
        s: Default::default(),
    }
}

mod import_analyzer;
mod utils;

static MODULE_SPECIFIER: Lazy<JsWord> = Lazy::new(|| "@swc/constify".into());

struct Constify {
    const_ctxt: SyntaxContext,
    s: State,
}

#[derive(Default)]
struct State {
    next_const_id: u32,

    vars: Vec<ConstItem>,

    imports: ImportMap,

    vars_declared_in_current_scope: FxHashSet<Id>,
}

struct ConstItem {
    decl: Decl,
    deps: FxHashSet<Id>,
}

impl Constify {
    fn next_var_name(&mut self, span: Span) -> Ident {
        let id = private_ident!(span, format!("__CONST_{}__", self.s.next_const_id));
        self.s.next_const_id += 1;
        id
    }

    fn visit_mut_stmt_likes<T>(&mut self, stmts: &mut Vec<T>)
    where
        T: StmtLike + VisitMutWith<Self>,
    {
        let mut new = vec![];

        for mut stmt in stmts.take() {
            stmt.visit_mut_with(self);

            new.extend(self.s.prepend_stmts.drain(..).map(T::from));

            new.push(stmt);
        }

        *stmts = new;
    }
}

impl VisitMut for Constify {
    noop_visit_mut_type!();

    fn visit_mut_expr(&mut self, e: &mut Expr) {
        e.visit_mut_children_with(self);

        if let Expr::Call(CallExpr {
            callee: Callee::Expr(callee),
            args,
            ..
        }) = e
        {
            if self
                .s
                .imports
                .is_import(&callee, &MODULE_SPECIFIER, "constify")
            {
                assert_eq!(args.len(), 1, "constify() takes exactly one argument");

                let var_name = self.next_var_name(callee.span());
                let decl = VarDeclarator {
                    span: DUMMY_SP,
                    name: var_name.clone().into(),
                    init: Some(args.pop().unwrap().expr),
                    definite: false,
                };
                let deps = ids_used_by(&decl.init);

                self.s.vars.push(ConstItem {
                    decl: Decl::Var(Box::new(VarDecl {
                        span: DUMMY_SP,
                        kind: VarDeclKind::Let,
                        declare: false,
                        decls: vec![decl],
                    })),
                    deps,
                });
                *e = Expr::Ident(var_name);
            } else if self
                .s
                .imports
                .is_import(&callee, &MODULE_SPECIFIER, "lazyConst")
            {
                assert_eq!(args.len(), 1, "lazyConst() takes exactly one argument");
            } else {
            };
        }
    }

    fn visit_mut_import_decl(&mut self, i: &mut ImportDecl) {
        i.visit_mut_children_with(self);
    }

    fn visit_mut_module(&mut self, m: &mut Module) {
        self.s.imports = ImportMap::analyze(m);
        if !self.s.imports.is_module_imported(&MODULE_SPECIFIER) {
            return;
        }

        m.visit_mut_children_with(self);
    }

    fn visit_mut_module_item(&mut self, s: &mut ModuleItem) {
        s.visit_mut_children_with(self);

        if let ModuleItem::ModuleDecl(swc_core::ecma::ast::ModuleDecl::Import(import)) = s {
            if import.src.value == *MODULE_SPECIFIER {
                s.take();
            }
        }
    }

    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        self.visit_mut_stmt_likes(stmts)
    }

    fn visit_mut_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        self.visit_mut_stmt_likes(stmts)
    }
}

#![feature(box_patterns)]

use std::hash::BuildHasherDefault;

use import_analyzer::ImportMap;
use indexmap::IndexSet;
use rustc_hash::{FxHashSet, FxHasher};
use swc_core::{
    common::{sync::Lazy, util::take::Take, Mark, Span, Spanned, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            CallExpr, Callee, Decl, Expr, Id, Ident, ImportDecl, Module, ModuleDecl, ModuleItem,
            Stmt, VarDecl, VarDeclKind, VarDeclarator,
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
}

struct ConstItem {
    decl: Option<Decl>,
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
        T: StmtLike + VisitMutWith<Self> + Vars,
    {
        let mut new = vec![];

        for mut stmt in stmts.take() {
            stmt.visit_mut_with(self);

            let vars_declared_by_stmt = stmt.vars_declared_by_item();

            for item in &mut self.s.vars {
                let mut did_work = false;

                for var_id in vars_declared_by_stmt.iter() {
                    item.deps.remove(var_id);
                    did_work = true;
                }

                if did_work && item.deps.is_empty() {
                    if let Some(decl) = item.decl.take() {
                        new.push(T::from_stmt(Stmt::Decl(decl)));
                    }
                }
            }

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
                    decl: Some(Decl::Var(Box::new(VarDecl {
                        span: DUMMY_SP,
                        kind: VarDeclKind::Let,
                        declare: false,
                        decls: vec![decl],
                    }))),
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

        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = s {
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

trait Vars {
    fn vars_declared_by_item(&self) -> Vec<Id>;
}

impl Vars for Stmt {
    fn vars_declared_by_item(&self) -> Vec<Id> {
        match self {
            Stmt::Decl(s) => match s {
                Decl::Class(s) => {}
                Decl::Fn(s) => {}
                Decl::Var(s) => {}
                Decl::Using(s) => {}
                _ => Default::default(),
            },
            _ => Default::default(),
        }
    }
}

impl Vars for ModuleItem {
    fn vars_declared_by_item(&self) -> Vec<Id> {
        match self {
            ModuleItem::ModuleDecl(s) => match s {
                ModuleDecl::Import(s) => {}
                ModuleDecl::ExportDecl(s) => {}
                ModuleDecl::ExportNamed(s) => {}
                ModuleDecl::ExportDefaultDecl(s) => {}
                ModuleDecl::ExportDefaultExpr(s) => {}
                ModuleDecl::ExportAll(s) => {}
                ModuleDecl::TsImportEquals(s) => {}
                ModuleDecl::TsExportAssignment(s) => {}
                ModuleDecl::TsNamespaceExport(s) => {}
            },
            ModuleItem::Stmt(s) => s.vars_declared_by_item(),
        }
    }
}

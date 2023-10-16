#![feature(box_patterns)]

use import_analyzer::ImportMap;
use once_cell::sync::Lazy;
use rustc_hash::FxHashSet;
use swc_atoms::JsWord;
use swc_common::{util::take::Take, Mark, Span, Spanned, SyntaxContext, DUMMY_SP};
use swc_ecma_ast::{
    op, ArrowExpr, AssignExpr, BlockStmt, CallExpr, Callee, Decl, DefaultDecl, Expr, FnDecl,
    FnExpr, Function, Id, Ident, ImportSpecifier, Module, ModuleDecl, ModuleItem, ReturnStmt, Stmt,
    VarDecl, VarDeclKind, VarDeclarator,
};
use swc_ecma_utils::{find_pat_ids, private_ident, StmtLike};
use swc_ecma_visit::{noop_visit_mut_type, VisitMut, VisitMutWith};
use tracing::debug;

use crate::utils::{ids_used_by, ids_used_by_ignoring_nested};

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
    name: Ident,
    decl: Option<Decl>,
    deps: FxHashSet<Id>,
}

impl Constify {
    fn next_var_name(&mut self, span: Span) -> Ident {
        let id = Ident::new(
            format!("__CONST_{}__", self.s.next_const_id).into(),
            span.with_ctxt(self.const_ctxt),
        );
        self.s.next_const_id += 1;
        id
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
                .is_import(callee, &MODULE_SPECIFIER, "constify")
            {
                assert_eq!(args.len(), 1, "constify() takes exactly one argument");

                let var_name = self.next_var_name(callee.span());
                let decl = VarDeclarator {
                    span: DUMMY_SP,
                    name: var_name.clone().into(),
                    init: Some(args.pop().unwrap().expr),
                    definite: false,
                };
                let deps = ids_used_by_ignoring_nested(&decl.init);

                self.s.vars.push(ConstItem {
                    name: var_name.clone(),
                    decl: Some(Decl::Var(Box::new(VarDecl {
                        span: DUMMY_SP,
                        kind: VarDeclKind::Const,
                        declare: false,
                        decls: vec![decl],
                    }))),
                    deps,
                });
                *e = Expr::Ident(var_name);
            } else if self
                .s
                .imports
                .is_import(callee, &MODULE_SPECIFIER, "lazyConst")
            {
                assert_eq!(args.len(), 1, "lazyConst() takes exactly one argument");

                let var_name = self.next_var_name(callee.span());
                let deps = ids_used_by(&args[0].expr);

                let data_var_name = private_ident!("__data__");

                let data_decl = VarDeclarator {
                    span: Span::default(),
                    name: data_var_name.clone().into(),
                    init: Some(args.pop().unwrap().expr),
                    definite: false,
                };

                let data_decl = Stmt::Decl(Decl::Var(Box::new(VarDecl {
                    span: DUMMY_SP,
                    kind: VarDeclKind::Const,
                    declare: false,
                    decls: vec![data_decl],
                })));

                let return_stmt = Stmt::Return(ReturnStmt {
                    span: DUMMY_SP,
                    arg: Some(Box::new(Expr::Assign(AssignExpr {
                        span: DUMMY_SP,
                        op: op!("="),
                        left: var_name.clone().into(),
                        right: Box::new(Expr::Fn(FnExpr {
                            ident: None,
                            function: Box::new(Function {
                                params: Default::default(),
                                decorators: Default::default(),
                                span: DUMMY_SP,
                                body: Some(BlockStmt {
                                    span: DUMMY_SP,
                                    stmts: {
                                        let s = Stmt::Return(ReturnStmt {
                                            span: DUMMY_SP,
                                            arg: Some(data_var_name.into()),
                                        });

                                        vec![s]
                                    },
                                }),
                                is_generator: false,
                                is_async: false,
                                type_params: Default::default(),
                                return_type: Default::default(),
                            }),
                        })),
                    }))),
                });

                let decl = Box::new(Function {
                    params: Default::default(),
                    decorators: Default::default(),
                    span: DUMMY_SP,
                    body: Some(BlockStmt {
                        span: DUMMY_SP,
                        stmts: vec![data_decl, return_stmt],
                    }),
                    is_generator: false,
                    is_async: false,
                    type_params: Default::default(),
                    return_type: Default::default(),
                });

                self.s.vars.push(ConstItem {
                    name: var_name.clone(),
                    decl: Some(Decl::Fn(FnDecl {
                        ident: var_name.clone(),
                        declare: false,
                        function: decl,
                    })),
                    deps,
                });
                *e = Expr::Ident(var_name);
            } else {
            };
        }
    }

    #[tracing::instrument(name = "Constify::visit", skip_all)]
    fn visit_mut_module(&mut self, m: &mut Module) {
        self.s.imports = ImportMap::analyze(m);
        if !self.s.imports.is_module_imported(&MODULE_SPECIFIER) {
            return;
        }

        m.visit_mut_children_with(self);

        if !self.s.vars.is_empty() {
            let _tracing = tracing::span!(tracing::Level::ERROR, "Constify::inject_vars").entered();
            m.visit_mut_with(&mut Injector {
                vars: self.s.vars.take(),
            });
        }
    }

    fn visit_mut_module_item(&mut self, s: &mut ModuleItem) {
        s.visit_mut_children_with(self);

        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = s {
            if import.src.value == *MODULE_SPECIFIER {
                s.take();
            }
        }
    }
}

trait Vars {
    fn vars_declared_by_item(&self) -> Vec<Id>;
}

impl Vars for Stmt {
    fn vars_declared_by_item(&self) -> Vec<Id> {
        match self {
            Stmt::Decl(s) => s.vars_declared_by_item(),
            _ => Default::default(),
        }
    }
}

impl Vars for Decl {
    fn vars_declared_by_item(&self) -> Vec<Id> {
        match self {
            Decl::Class(s) => {
                vec![s.ident.to_id()]
            }
            Decl::Fn(s) => {
                vec![s.ident.to_id()]
            }
            Decl::Var(s) => find_pat_ids(&s.decls),
            Decl::Using(s) => find_pat_ids(&s.decls),
            _ => Default::default(),
        }
    }
}

impl Vars for ModuleDecl {
    fn vars_declared_by_item(&self) -> Vec<Id> {
        match self {
            ModuleDecl::Import(s) => {
                let mut buf = vec![];

                for s in s.specifiers.iter() {
                    match s {
                        ImportSpecifier::Named(s) => {
                            buf.push(s.local.to_id());
                        }
                        ImportSpecifier::Default(s) => {
                            buf.push(s.local.to_id());
                        }
                        ImportSpecifier::Namespace(s) => {
                            buf.push(s.local.to_id());
                        }
                    }
                }

                buf
            }
            ModuleDecl::ExportDecl(s) => s.decl.vars_declared_by_item(),
            ModuleDecl::ExportDefaultDecl(s) => match &s.decl {
                DefaultDecl::Class(d) => d.ident.iter().map(|i| i.to_id()).collect(),
                DefaultDecl::Fn(d) => d.ident.iter().map(|i| i.to_id()).collect(),

                _ => Default::default(),
            },
            _ => Default::default(),
        }
    }
}

impl Vars for ModuleItem {
    fn vars_declared_by_item(&self) -> Vec<Id> {
        match self {
            ModuleItem::ModuleDecl(s) => s.vars_declared_by_item(),
            ModuleItem::Stmt(s) => s.vars_declared_by_item(),
        }
    }
}

struct Injector {
    vars: Vec<ConstItem>,
}

impl Injector {
    fn declare_scope_vars(&mut self, vars: Vec<Id>) {
        for var_id in vars {
            for item in &mut self.vars {
                item.deps.remove(&var_id);
            }
        }
    }

    fn visit_mut_stmt_likes<T>(&mut self, stmts: &mut Vec<T>)
    where
        T: StmtLike + VisitMutWith<Self> + Vars,
    {
        let mut buf = vec![];

        for item in &mut self.vars {
            if item.deps.is_empty() {
                if let Some(decl) = item.decl.take() {
                    buf.push(T::from_stmt(Stmt::Decl(decl)));
                }
            }
        }

        for mut stmt in stmts.take() {
            stmt.visit_mut_with(self);

            let vars_declared_by_stmt = stmt.vars_declared_by_item();

            for item in &mut self.vars {
                for var_id in vars_declared_by_stmt.iter() {
                    item.deps.remove(var_id);
                }

                if item.deps.is_empty() {
                    if let Some(decl) = item.decl.take() {
                        buf.push(T::from_stmt(Stmt::Decl(decl)));
                    }
                } else {
                    debug!("{} is not ready: {:?}", item.name.sym, item.deps);
                }
            }

            buf.push(stmt);
        }

        *stmts = buf;
    }
}

impl VisitMut for Injector {
    noop_visit_mut_type!();

    fn visit_mut_arrow_expr(&mut self, n: &mut ArrowExpr) {
        self.declare_scope_vars(find_pat_ids(&n.params));

        n.visit_mut_children_with(self);
    }

    fn visit_mut_function(&mut self, n: &mut Function) {
        self.declare_scope_vars(find_pat_ids(&n.params));

        n.visit_mut_children_with(self);
    }

    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        self.visit_mut_stmt_likes(stmts)
    }

    fn visit_mut_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        self.visit_mut_stmt_likes(stmts)
    }
}

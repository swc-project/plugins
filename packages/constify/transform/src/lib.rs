#![feature(box_patterns)]

use import_analyzer::ImportMap;
use swc_core::{
    common::{sync::Lazy, util::take::Take, Mark, Span, Spanned, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            CallExpr, Callee, Decl, Expr, Ident, ImportDecl, Module, ModuleItem, Stmt, VarDecl,
            VarDeclKind, VarDeclarator,
        },
        atoms::JsWord,
        utils::private_ident,
        visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
    },
};

pub fn constify() -> impl VisitMut {
    Constify {
        const_ctxt: SyntaxContext::empty().apply_mark(Mark::new()),
        next_const_id: 0,
        prepend_stmts: vec![],
        imports: Default::default(),
    }
}

mod import_analyzer;

static MODULE_SPECIFIER: Lazy<JsWord> = Lazy::new(|| "@swc/constify".into());

struct Constify {
    const_ctxt: SyntaxContext,

    next_const_id: u32,

    prepend_stmts: Vec<Stmt>,

    imports: ImportMap,
}

impl Constify {
    fn next_var_name(&mut self, span: Span) -> Ident {
        let id = private_ident!(span, format!("__CONST_{}__", self.next_const_id));
        self.next_const_id += 1;
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
                self.prepend_stmts
                    .push(Stmt::Decl(Decl::Var(Box::new(VarDecl {
                        span: DUMMY_SP,
                        kind: VarDeclKind::Let,
                        declare: false,
                        decls: vec![decl],
                    }))));
                *e = Expr::Ident(var_name);
            } else if self
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
        self.imports = ImportMap::analyze(m);
        if !self.imports.is_module_imported(&MODULE_SPECIFIER) {
            return;
        }

        m.visit_mut_children_with(self);
    }

    fn visit_mut_module_item(&mut self, s: &mut ModuleItem) {
        s.visit_mut_children_with(self);
    }

    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        let mut new = vec![];

        for mut stmt in stmts.take() {
            stmt.visit_mut_with(self);

            new.extend(self.prepend_stmts.drain(..).map(ModuleItem::from));

            new.push(stmt);
        }

        *stmts = new;
    }

    fn visit_mut_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        let mut new = vec![];

        for mut stmt in stmts.take() {
            stmt.visit_mut_with(self);

            new.append(&mut self.prepend_stmts);

            new.push(stmt);
        }

        *stmts = new;
    }
}

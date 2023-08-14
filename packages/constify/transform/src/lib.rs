use swc_core::{
    common::{Mark, SyntaxContext},
    ecma::{
        ast::{Expr, ImportDecl, Stmt},
        visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
    },
};

pub fn constify() -> impl VisitMut {
    Constify {
        const_ctxt: SyntaxContext::empty().apply_mark(Mark::new()),
        next_const_id: 0,
        prepend_stmts: vec![],
    }
}

struct Constify {
    const_ctxt: SyntaxContext,

    next_const_id: u32,

    prepend_stmts: Vec<Stmt>,
}

impl VisitMut for Constify {
    noop_visit_mut_type!();

    fn visit_mut_import_decl(&mut self, i: &mut ImportDecl) {
        i.visit_mut_children_with(self);
    }

    fn visit_mut_expr(&mut self, e: &mut Expr) {
        e.visit_mut_children_with(self);
    }
}

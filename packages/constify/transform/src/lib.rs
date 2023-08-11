use swc_core::{
    common::{Mark, SyntaxContext},
    ecma::{
        ast::Stmt,
        visit::{noop_visit_mut_type, VisitMut},
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
}

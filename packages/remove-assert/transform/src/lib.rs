use serde::Deserialize;
use swc_common::{SyntaxContext, DUMMY_SP};
use swc_ecma_ast::*;
use swc_ecma_visit::{fold_pass, noop_fold_type, Fold, FoldWith};

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Config {
    All(bool),
    WithOptions(Options),
}

impl Config {
    pub fn truthy(&self) -> bool {
        match self {
            Config::All(b) => *b,
            Config::WithOptions(_) => true,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Options {}

struct RemoveAssert {
    unresolved_ctxt: SyntaxContext,
}

impl RemoveAssert {
    fn is_global_assert(&self, ident: &Ident) -> bool {
        &ident.sym == "assert" && ident.ctxt == self.unresolved_ctxt
    }

    fn should_remove_call(&self, n: &CallExpr) -> bool {
        let callee = &n.callee;
        match callee {
            Callee::Expr(e) => match &**e {
                Expr::Ident(i) if self.is_global_assert(i) => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl Fold for RemoveAssert {
    noop_fold_type!();

    fn fold_stmt(&mut self, stmt: Stmt) -> Stmt {
        if let Stmt::Expr(e) = &stmt {
            if let Expr::Call(c) = &*e.expr {
                if self.should_remove_call(c) {
                    return Stmt::Empty(EmptyStmt { span: DUMMY_SP });
                }
            }
        }
        stmt.fold_children_with(self)
    }
}

pub fn remove_assert(_config: Config, unresolved_ctxt: SyntaxContext) -> impl Pass {
    fold_pass(RemoveAssert { unresolved_ctxt })
}

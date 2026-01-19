use serde::Deserialize;
use swc_atoms::Atom;
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
pub struct Options {
    #[serde(default)]
    pub exclude: Vec<Atom>,
}

struct RemoveAssert {
    exclude: Vec<Atom>,
    unresolved_ctxt: SyntaxContext,
}

impl RemoveAssert {
    fn is_global_assert(&self, ident: &Ident) -> bool {
        &ident.sym == "assert" && ident.ctxt == self.unresolved_ctxt
    }

    fn should_remove_call(&mut self, n: &CallExpr) -> bool {
        let callee = &n.callee;

        match callee {
            // Direct call to assert()
            Callee::Expr(e) => match &**e {
                Expr::Ident(i) if self.is_global_assert(i) => {
                    // Check if we should exclude this call
                    // For assert, we don't have methods like console.log, so exclude is less relevant
                    // But we keep it for consistency with the API
                    return true;
                }
                Expr::Member(m) => {
                    // Handle assert.* calls (e.g., assert.ok, assert.equal, etc.)
                    // Don't attempt to evaluate computed properties.
                    if matches!(&m.prop, MemberProp::Computed(..)) {
                        return false;
                    }

                    // Only proceed if the object is the global `assert` object.
                    match &*m.obj {
                        Expr::Ident(i) if self.is_global_assert(i) => {}
                        _ => return false,
                    }

                    // Check if the property is requested to be excluded.
                    // Here we do an O(n) search on the list of excluded properties because the size
                    // should be small.
                    match &m.prop {
                        MemberProp::Ident(i) if !self.exclude.contains(&i.sym) => {}
                        _ => return false,
                    }

                    true
                }
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

pub fn remove_assert(config: Config, unresolved_ctxt: SyntaxContext) -> impl Pass {
    let exclude = match config {
        Config::WithOptions(x) => x.exclude,
        _ => vec![],
    };
    fold_pass(RemoveAssert {
        exclude,
        unresolved_ctxt,
    })
}

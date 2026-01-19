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
    /// SyntaxContexts of identifiers imported from 'assert' or 'node:assert'
    assert_import_ctxts: Vec<SyntaxContext>,
}

impl RemoveAssert {
    fn is_assert_module(src: &str) -> bool {
        src == "assert"
            || src == "node:assert"
            || src == "assert/strict"
            || src == "node:assert/strict"
    }

    fn collect_assert_imports(&mut self, module: &Module) {
        for item in &module.body {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item {
                let src = match import.src.value.as_str() {
                    Some(s) => s,
                    None => continue,
                };
                if !Self::is_assert_module(src) {
                    continue;
                }
                for specifier in &import.specifiers {
                    match specifier {
                        // import assert from 'assert'
                        ImportSpecifier::Default(default) => {
                            self.assert_import_ctxts.push(default.local.ctxt);
                        }
                        // import * as assert from 'assert'
                        ImportSpecifier::Namespace(ns) => {
                            self.assert_import_ctxts.push(ns.local.ctxt);
                        }
                        // import { assert, fail, ok } from 'assert'
                        ImportSpecifier::Named(named) => {
                            self.assert_import_ctxts.push(named.local.ctxt);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn is_assert_call(&self, ident: &Ident) -> bool {
        // Check if it's a global unresolved assert
        if &ident.sym == "assert" && ident.ctxt == self.unresolved_ctxt {
            return true;
        }
        // Check if it's an imported assert identifier
        self.assert_import_ctxts.contains(&ident.ctxt)
    }

    fn should_remove_call(&self, n: &CallExpr) -> bool {
        let callee = &n.callee;
        match callee {
            Callee::Expr(e) => match &**e {
                Expr::Ident(i) if self.is_assert_call(i) => true,
                // Handle assert.ok(), assert.strictEqual(), etc.
                Expr::Member(member) => {
                    if let Expr::Ident(obj) = &*member.obj {
                        self.is_assert_call(obj)
                    } else {
                        false
                    }
                }
                _ => false,
            },
            _ => false,
        }
    }
}

impl Fold for RemoveAssert {
    noop_fold_type!();

    fn fold_module(&mut self, module: Module) -> Module {
        // First pass: collect assert imports
        self.collect_assert_imports(&module);
        // Then fold the module
        module.fold_children_with(self)
    }

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
    fold_pass(RemoveAssert {
        unresolved_ctxt,
        assert_import_ctxts: Vec::new(),
    })
}

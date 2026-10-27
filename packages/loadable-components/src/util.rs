use swc_core::ecma::ast::*;

pub(crate) fn get_import_arg(call: &CallExpr) -> &Expr {
    &call.args[0].expr
}

/// `quote!` can only produce a `Stmt`, so quoted function bodies are parsed as
/// a block statement and converted here.
pub(crate) fn expect_fn_body(stmt: Stmt) -> FunctionBody {
    let block = stmt.expect_block();

    FunctionBody {
        span: block.span,
        stmts: block.stmts,
    }
}

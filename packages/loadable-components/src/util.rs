use swc_core::ecma::ast::*;

pub(crate) fn get_import_arg(call: &CallExpr) -> &Expr {
    &call.args[0].expr
}

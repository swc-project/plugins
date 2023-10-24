use swc_common::Span;
use swc_ecma_ast::*;

pub struct ExternalStyle {
    pub expr: Expr,
    pub identifier: Ident,
    pub is_global: bool,
}

pub struct LocalStyle {
    pub hash: String,
    pub css: String,
    pub css_span: Span,
    pub is_dynamic: bool,
    #[allow(clippy::vec_box)]
    pub expressions: Vec<Box<Expr>>,

    /// If true, `format!("__styled-jsx-placeholder-{}__: 0", i)` is used.
    pub is_expr_property: Vec<bool>,
}

pub enum JSXStyle {
    Local(LocalStyle),
    External(ExternalStyle),
}

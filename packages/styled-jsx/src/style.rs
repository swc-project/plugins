use swc_core::{common::Span, ecma::ast::*};

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
}

pub enum JSXStyle {
    Local(LocalStyle),
    External(ExternalStyle),
}

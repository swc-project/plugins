use std::borrow::Cow;
use swc_atoms::js_word;
use swc_ecmascript::{ast::*, utils::ExprExt};

pub(crate) fn get_prop_key_as_expr(p: &Prop) -> Cow<Expr> {
    match p {
        Prop::Shorthand(p) => Cow::Owned(Expr::Ident(p.clone())),
        Prop::KeyValue(p) => prop_name_to_expr(&p.key),
        Prop::Assign(p) => Cow::Owned(Expr::Ident(p.key.clone())),
        Prop::Getter(p) => prop_name_to_expr(&p.key),
        Prop::Setter(p) => prop_name_to_expr(&p.key),
        Prop::Method(p) => prop_name_to_expr(&p.key),
    }
}

pub(crate) fn prop_name_to_expr(p: &PropName) -> Cow<Expr> {
    match p {
        PropName::Ident(p) => Cow::Owned(Expr::Ident(p.clone())),
        PropName::Str(p) => Cow::Owned(Expr::Lit(Lit::Str(p.clone()))),
        PropName::Num(p) => Cow::Owned(Expr::Lit(Lit::Num(p.clone()))),
        PropName::BigInt(p) => Cow::Owned(Expr::Lit(Lit::BigInt(p.clone()))),
        PropName::Computed(e) => Cow::Borrowed(&e.expr),
    }
}

pub(crate) fn get_prop_name(p: &Prop) -> Option<&PropName> {
    match p {
        Prop::Shorthand(..) => None,
        Prop::KeyValue(p) => Some(&p.key),
        Prop::Assign(..) => None,
        Prop::Getter(p) => Some(&p.key),
        Prop::Setter(p) => Some(&p.key),
        Prop::Method(p) => Some(&p.key),
    }
}

pub(crate) fn is_styled(tag: &Expr) -> bool {
    match tag {
        Expr::Call(CallExpr {
            callee: ExprOrSuper::Expr(callee),
            ..
        }) => match &**callee {
            Expr::Member(MemberExpr {
                computed: false,
                obj: ExprOrSuper::Expr(obj),
                prop,
                ..
            }) => {
                if !prop.is_ident_ref_to(js_word!("default")) {
                    return is_styled(&obj);
                }
            }
        },

        _ => {}
    }

    false
}

use swc_core::{
    atoms::Atom,
    common::{Span, DUMMY_SP},
    ecma::{
        ast::*,
        utils::{member_expr, private_ident, quote_ident, quote_str, ExprFactory},
    },
};

use crate::local_export_strip::Export;

/// ```javascript
/// {
///     get() { return ident; }
/// }
/// ```
pub(crate) fn prop_method_getter(ident: Ident) -> Prop {
    let key = quote_ident!("get").into();

    MethodProp {
        key,
        function: ident.into_lazy_fn(Default::default()).into(),
    }
    .into()
}

/// ```javascript
/// {
///     set(v) { ident = v; }
/// }
/// ```
pub(crate) fn prop_method_setter(ident: Ident) -> Prop {
    let key = quote_ident!("set").into();

    let setter_param = private_ident!("v");
    let params = vec![setter_param.clone().into()];

    let body = BlockStmt {
        stmts: vec![setter_param
            .make_assign_to(op!("="), ident.clone().into())
            .into_stmt()],
        ..Default::default()
    };

    MethodProp {
        key,
        function: Function {
            params,
            body: Some(body),
            ..Default::default()
        }
        .into(),
    }
    .into()
}

/// Creates
///
///```js
/// 
///  Object.defineProperty(target, prop_name, {
///      ...props
///  });
/// ```
pub(super) fn object_define_property(
    target: ExprOrSpread,
    prop_name: ExprOrSpread,
    descriptor: ExprOrSpread,
) -> Expr {
    member_expr!(Default::default(), DUMMY_SP, Object.defineProperty)
        .as_call(DUMMY_SP, vec![target, prop_name, descriptor])
}

pub(crate) fn object_define_enumerable_configurable(
    target: ExprOrSpread,
    prop_name: ExprOrSpread,
    getter: PropOrSpread,
    setter: PropOrSpread,
) -> Expr {
    object_define_property(
        target,
        prop_name,
        ObjectLit {
            span: DUMMY_SP,
            props: vec![
                PropOrSpread::Prop(Box::new(
                    KeyValueProp {
                        key: quote_ident!("enumerable").into(),
                        value: Box::new(true.into()),
                    }
                    .into(),
                )),
                getter,
                setter,
                PropOrSpread::Prop(Box::new(
                    KeyValueProp {
                        key: quote_ident!("configurable").into(),
                        value: Box::new(true.into()),
                    }
                    .into(),
                )),
            ],
        }
        .as_arg(),
    )
}

pub(crate) fn emit_export_stmts(exports: Ident, export: Export) -> Vec<Stmt> {
    export
        .into_iter()
        .map(|(export_name, export_item)| {
            let prop_name = quote_str!(export_item.export_name_span(), export_name);
            let local_ident = export_item.into_local_ident();

            object_define_enumerable_configurable(
                exports.clone().as_arg(),
                prop_name.as_arg(),
                prop_method_getter(local_ident.clone()).into(),
                prop_method_setter(local_ident).into(),
            )
            .into_stmt()
        })
        .collect()
}

pub(crate) fn key_from_export_name(n: &ModuleExportName) -> (Atom, Span) {
    match n {
        ModuleExportName::Ident(ident) => (ident.sym.clone(), ident.span),
        ModuleExportName::Str(s) => (
            match s.value.as_atom() {
                Some(s) => s.clone(),
                None => panic!("non-utf8 export name: {:?}", s.value),
            },
            s.span,
        ),
    }
}

pub(crate) fn local_ident_from_export_name(n: ModuleExportName) -> Ident {
    let name = match n {
        ModuleExportName::Ident(ident) => ident.sym,
        ModuleExportName::Str(s) => match s.value.as_atom() {
            Some(s) => s.clone(),
            None => panic!("non-utf8 export name: {:?}", s.value),
        },
    };

    match Ident::verify_symbol(&name) {
        Ok(_) => private_ident!(name),
        Err(s) => private_ident!(s),
    }
}

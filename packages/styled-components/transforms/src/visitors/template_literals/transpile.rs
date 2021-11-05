//! Port of https://github.com/styled-components/babel-plugin-styled-components/blob/a20c3033508677695953e7a434de4746168eeb4e/src/visitors/transpileCssProp.js

use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;
use swc_atoms::{js_word, JsWord};
use swc_common::{util::take::Take, Spanned, DUMMY_SP};
use swc_ecmascript::{
    ast::*,
    utils::{prepend, private_ident, quote_ident, quote_str, ExprExt, ExprFactory},
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
};

static TAG_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new("^[a-z][a-z\\d]*(\\-[a-z][a-z\\d]*)?$").unwrap());

pub fn transpile_css_prop() -> impl Fold + VisitMut {
    as_folder(TranspileCssProp::default())
}

#[derive(Default)]
struct TranspileCssProp {
    import_name: Option<Ident>,
}

impl VisitMut for TranspileCssProp {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        // TODO: Skip if there are no css prop usage
        n.visit_mut_children_with(self);

        if let Some(import_name) = self.import_name.take() {
            let specifier = ImportSpecifier::Default(ImportDefaultSpecifier {
                span: DUMMY_SP,
                local: import_name,
            });
            prepend(
                &mut n.body,
                ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                    span: DUMMY_SP,
                    specifiers: vec![specifier],
                    src: Str {
                        span: DUMMY_SP,
                        value: "styled-components".into(),
                        has_escape: Default::default(),
                        kind: Default::default(),
                    },
                    type_only: Default::default(),
                    asserts: Default::default(),
                })),
            );
        }
    }

    fn visit_mut_jsx_element(&mut self, elem: &mut JSXElement) {
        elem.visit_mut_children_with(self);

        let mut extra_attrs = vec![];

        for attr in elem.opening.attrs.iter_mut() {
            match &mut *attr {
                JSXAttrOrSpread::JSXAttr(attr) => {
                    if !matches!(&attr.name, JSXAttrName::Ident(i) if &*i.sym == "css") {
                        continue;
                    }

                    let import_name = self
                        .import_name
                        .get_or_insert_with(|| private_ident!("styled"))
                        .clone();

                    let mut name = get_name(&elem.opening.name);
                    let id_sym = get_first_letter_uppercased(&name);

                    let id: Ident =
                        private_ident!(elem.opening.name.span(), format!("Styled{}", id_sym));

                    let (styled, injector) = if TAG_NAME_REGEX.is_match(&name) {
                        (
                            (Expr::Call(CallExpr {
                                span: DUMMY_SP,
                                callee: import_name.as_callee(),
                                args: vec![Lit::Str(Str {
                                    span: DUMMY_SP,
                                    value: name.into(),
                                    has_escape: false,
                                    kind: Default::default(),
                                })
                                .as_arg()],
                                type_args: Default::default(),
                            })),
                            None::<()>,
                        )
                    } else {
                        // let name_expr = get_name_expr(&n.opening.name);

                        // TODO

                        return;
                    };

                    let mut css = match &mut attr.value {
                        Some(css) => {
                            //

                            match css {
                                JSXAttrValue::Lit(Lit::Str(v)) => Expr::Tpl(Tpl {
                                    span: DUMMY_SP,
                                    exprs: Default::default(),
                                    quasis: vec![TplElement {
                                        span: DUMMY_SP,
                                        tail: true,
                                        cooked: None,
                                        raw: v.clone(),
                                    }],
                                }),
                                JSXAttrValue::JSXExprContainer(JSXExprContainer {
                                    expr: JSXExpr::Expr(v),
                                    ..
                                }) => match &mut **v {
                                    Expr::Tpl(..) => *v.take(),
                                    // TODO: Check if `path.node.value.expression.quasi` is array
                                    // Expr::TaggedTpl(v) if v.tag.is_ident_ref_to("css".into()) =>
                                    // {     v.tpl.quasis[0].
                                    // take() }
                                    Expr::Object(..) => *v.take(),
                                    _ => Expr::Tpl(Tpl {
                                        span: DUMMY_SP,
                                        exprs: vec![v.take()],
                                        quasis: vec![
                                            TplElement {
                                                span: DUMMY_SP,
                                                tail: false,
                                                cooked: None,
                                                raw: quote_str!(""),
                                            },
                                            TplElement {
                                                span: DUMMY_SP,
                                                tail: true,
                                                cooked: None,
                                                raw: quote_str!(""),
                                            },
                                        ],
                                    }),
                                },

                                _ => return,
                            }
                        }
                        None => return,
                    };

                    attr.name = JSXAttrName::Ident(Take::dummy());
                    // TODO: Remove this attribute

                    elem.opening.name = JSXElementName::Ident(id.clone());

                    if let Some(closing) = &mut elem.closing {
                        closing.name = JSXElementName::Ident(id.clone());
                    }

                    // object syntax
                    if let Expr::Object(css_obj) = &mut css {
                        // Original plugin says
                        //
                        //
                        // for objects as CSS props, we have to recurse through the object and
                        // replace any object key/value scope references with generated props
                        // similar to how the template literal transform above creates dynamic
                        // interpolations
                        let p = quote_ident!("p");

                        let mut reducer = PropertyReducer {
                            p: p.clone(),
                            replace_object_with_prop_function: false,
                            extra_attrs: Default::default(),
                        };

                        css_obj.props = css_obj
                            .props
                            .take()
                            .into_iter()
                            .fold(vec![], |acc, property| {
                                reducer.reduce_object_properties(acc, property)
                            });

                        extra_attrs.extend(reducer.extra_attrs);

                        if reducer.replace_object_with_prop_function {
                            css = Expr::Arrow(ArrowExpr {
                                span: DUMMY_SP,
                                params: vec![Pat::Ident(p.clone().into())],
                                body: BlockStmtOrExpr::Expr(Box::new(css.take())),
                                is_async: false,
                                is_generator: false,
                                type_params: Default::default(),
                                return_type: Default::default(),
                            });
                        }
                    } else {
                        // tagged template literal
                        // let mut tpl = css.expect_tagged_tpl();
                    }
                }
                JSXAttrOrSpread::SpreadElement(_) => {}
            }
        }

        elem.opening.attrs.retain(|attr| {
            match attr {
                JSXAttrOrSpread::JSXAttr(attr) => {
                    if matches!(
                        attr.name,
                        JSXAttrName::Ident(Ident {
                            sym: js_word!(""),
                            ..
                        })
                    ) {
                        return false;
                    }
                }
                JSXAttrOrSpread::SpreadElement(_) => {}
            }
            true
        });

        elem.opening.attrs.extend(extra_attrs);
    }
}

struct PropertyReducer {
    p: Ident,
    replace_object_with_prop_function: bool,
    extra_attrs: Vec<JSXAttrOrSpread>,
}

impl PropertyReducer {
    fn reduce_object_properties(
        &mut self,
        mut acc: Vec<PropOrSpread>,
        mut property: PropOrSpread,
    ) -> Vec<PropOrSpread> {
        match property {
            PropOrSpread::Spread(ref mut prop) => {
                // handle spread variables and such

                if let Expr::Object(arg) = &mut *prop.expr {
                    arg.props = arg
                        .props
                        .take()
                        .into_iter()
                        .fold(vec![], |acc, p| self.reduce_object_properties(acc, p));
                } else {
                    self.replace_object_with_prop_function = true;

                    let identifier = get_local_identifier(&prop.expr);

                    self.extra_attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
                        span: DUMMY_SP,
                        name: JSXAttrName::Ident(identifier.clone()),
                        value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                            span: DUMMY_SP,
                            expr: JSXExpr::Expr(prop.expr.take()),
                        })),
                    }));

                    prop.expr = Box::new(self.p.clone().make_member(identifier));
                }

                acc.push(property);
            }
            PropOrSpread::Prop(ref mut prop) => {
                let key = get_prop_key_as_expr(&prop);

                if key.is_member()
                    || key.is_call()
                    || (key.is_ident() && !matches!(&**prop, Prop::Shorthand(..)))
                {
                    self.replace_object_with_prop_function = true;

                    let identifier = get_local_identifier(&key);

                    self.extra_attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
                        span: DUMMY_SP,
                        name: identifier.clone().into(),
                        value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                            span: DUMMY_SP,
                            // TODO: Perf
                            expr: JSXExpr::Expr(Box::new(key.clone().into_owned())),
                        })),
                    }));

                    set_key_of_prop(prop, Box::new(self.p.clone().make_member(identifier)));
                }

                let mut value = take_prop_value(prop);

                if let Expr::Object(value) = &mut *value {
                    value.props = value
                        .props
                        .take()
                        .into_iter()
                        .fold(vec![], |acc, p| self.reduce_object_properties(acc, p));
                    acc.push(property);
                } else if !matches!(&*value, Expr::Lit(..)) {
                    // if a non-primitive value we have to interpolate it

                    self.replace_object_with_prop_function = true;

                    let identifier = get_local_identifier(&value);

                    self.extra_attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
                        span: DUMMY_SP,
                        name: JSXAttrName::Ident(identifier.clone()),
                        value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                            span: DUMMY_SP,
                            expr: JSXExpr::Expr(value.take()),
                        })),
                    }));

                    let key = get_prop_key_as_expr(&prop);

                    acc.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                        key: PropName::Computed(ComputedPropName {
                            span: DUMMY_SP,
                            expr: Box::new(key.into_owned()),
                        }),
                        value: Box::new(self.p.clone().make_member(identifier)),
                    }))));
                } else {
                    acc.push(property);
                }
            }
        }

        acc
    }
}

fn take_prop_value(prop: &mut Prop) -> Box<Expr> {
    match prop {
        Prop::Shorthand(p) => Box::new(Expr::Ident(p.clone())),
        Prop::KeyValue(p) => p.value.take(),
        Prop::Assign(..) => unreachable!("assign property is not allowed for object literals"),
        Prop::Getter(p) => todo!(),
        Prop::Setter(p) => todo!(),
        Prop::Method(p) => todo!(),
    }
}

fn set_key_of_prop(prop: &mut Prop, key: Box<Expr>) {
    let value = take_prop_value(prop);

    *prop = Prop::KeyValue(KeyValueProp {
        key: PropName::Computed(ComputedPropName {
            span: DUMMY_SP,
            expr: key,
        }),
        value,
    });
}

fn get_local_identifier(expr: &Expr) -> Ident {
    let identifier = private_ident!(expr.span(), "css");

    identifier
}

fn get_prop_key_as_expr(p: &Prop) -> Cow<Expr> {
    match p {
        Prop::Shorthand(p) => Cow::Owned(Expr::Ident(p.clone())),
        Prop::KeyValue(p) => prop_name_to_expr(&p.key),
        Prop::Assign(p) => Cow::Owned(Expr::Ident(p.key.clone())),
        Prop::Getter(p) => prop_name_to_expr(&p.key),
        Prop::Setter(p) => prop_name_to_expr(&p.key),
        Prop::Method(p) => prop_name_to_expr(&p.key),
    }
}

fn prop_name_to_expr(p: &PropName) -> Cow<Expr> {
    match p {
        PropName::Ident(p) => Cow::Owned(Expr::Ident(p.clone())),
        PropName::Str(p) => Cow::Owned(Expr::Lit(Lit::Str(p.clone()))),
        PropName::Num(p) => Cow::Owned(Expr::Lit(Lit::Num(p.clone()))),
        PropName::BigInt(p) => Cow::Owned(Expr::Lit(Lit::BigInt(p.clone()))),
        PropName::Computed(e) => Cow::Borrowed(&e.expr),
    }
}

fn get_first_letter_uppercased(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn get_name(el: &JSXElementName) -> JsWord {
    match el {
        JSXElementName::Ident(v) => v.sym.clone(),
        JSXElementName::JSXMemberExpr(e) => {
            format!("{}.{}", get_name_of_jsx_obj(&e.obj), e.prop.sym).into()
        }
        _ => {
            unimplemented!("get_name for namespaced jsx element")
        }
    }
}

fn get_name_of_jsx_obj(el: &JSXObject) -> JsWord {
    match el {
        JSXObject::Ident(v) => v.sym.clone(),
        JSXObject::JSXMemberExpr(e) => {
            format!("{}.{}", get_name_of_jsx_obj(&e.obj), e.prop.sym).into()
        }
        _ => {
            unimplemented!("get_name for namespaced jsx element")
        }
    }
}

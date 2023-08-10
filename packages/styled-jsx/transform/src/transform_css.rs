use std::{borrow::Cow, convert::Infallible, panic, sync::Arc};

use easy_error::{bail, Error, ResultExt};
use lightningcss::{
    css_modules::Pattern,
    properties::custom::{TokenList, TokenOrValue},
    selector::{Combinator, Component, PseudoClass, Selector, SelectorList},
    stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet},
    traits::{ParseWithOptions, ToCss},
    values::ident::Ident,
    visit_types,
    visitor::{Visit, VisitTypes, Visitor},
};
use parcel_selectors::{parser::SelectorIter, SelectorImpl};
use swc_core::{
    common::{
        errors::HANDLER, source_map::Pos, util::take::Take, BytePos, SourceMap, Span, Spanned,
        SyntaxContext, DUMMY_SP,
    },
    ecma::{
        ast::{Expr, Tpl, TplElement},
        parser::StringInput,
    },
};
use tracing::{debug, trace};

use crate::{
    style::LocalStyle,
    utils::{hash_string, string_literal_expr},
};

pub fn transform_css(
    _cm: Arc<SourceMap>,
    style_info: &LocalStyle,
    is_global: bool,
    class_name: &Option<String>,
) -> Result<Expr, Error> {
    debug!("CSS: \n{}", style_info.css);

    let result: Result<StyleSheet, _> = StyleSheet::parse(
        &style_info.css,
        ParserOptions {
            // We cannot use css_modules for `:global` because lightningcss does not support
            // parsing-only mode.
            css_modules: None,
            ..Default::default()
        },
    );
    let mut ss = match result {
        Ok(ss) => ss,
        Err(err) => {
            HANDLER.with(|handler| {
                // Print css parsing errors
                // TODO:
                // err.to_diagnostics(handler).emit();

                // TODO(kdy1): We may print css so the user can see the error, and report it.

                handler
                    .struct_span_err(
                        style_info.css_span,
                        "Failed to parse css in styled jsx component",
                    )
                    .note(&format!("Input to the css parser is {}", style_info.css))
                    .emit()
            });
            bail!("Failed to parse css");
        }
    };

    // Apply auto prefixer
    // TODO:
    ss.minify(MinifyOptions {
        ..Default::default()
    })
    .expect("failed to minify/auto-prefix css");
    ss.visit(&mut Namespacer {
        class_name: match class_name {
            Some(s) => s.clone(),
            None => format!("jsx-{}", &hash_string(&style_info.hash)),
        },
        is_global,
        is_dynamic: style_info.is_dynamic,
    })
    .expect("failed to transform css");

    let res = ss
        .to_css(PrinterOptions {
            minify: true,
            // TODO
            // targets: (),
            ..Default::default()
        })
        .context("failed to print css")?;

    debug!("Transformed CSS: \n{}", res.code);

    if style_info.expressions.is_empty() {
        return Ok(string_literal_expr(&res.code));
    }

    let mut parts: Vec<&str> = res.code.split("__styled-jsx-placeholder-").collect();
    let mut final_expressions = vec![];
    for i in parts.iter_mut().skip(1) {
        let (num_len, expression_index) = read_number(i);
        final_expressions.push(style_info.expressions[expression_index].clone());
        let substring = &i[(num_len + 2)..];
        *i = substring;
    }

    Ok(Expr::Tpl(Tpl {
        quasis: parts
            .iter()
            .map(|quasi| {
                TplElement {
                    cooked: None, // ? Do we need cooked as well
                    raw: quasi.replace('`', "\\`").into(),
                    span: DUMMY_SP,
                    tail: false,
                }
            })
            .collect(),
        exprs: final_expressions,
        span: DUMMY_SP,
    }))
}

/// Returns `(length, value)`
fn read_number(s: &str) -> (usize, usize) {
    for (idx, c) in s.char_indices() {
        if c.is_ascii_digit() {
            continue;
        }

        // For 10, we reach here after `0`.
        let value = s[0..idx].parse().expect("failed to parse");

        return (idx, value);
    }

    unreachable!("read_number(`{}`) is invalid because it is empty", s)
}

struct Namespacer {
    class_name: String,
    is_global: bool,
    is_dynamic: bool,
}

impl<'i> Visitor<'i> for Namespacer {
    type Error = Infallible;

    const TYPES: VisitTypes = visit_types!(SELECTORS);

    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn visit_selector(&mut self, selector: &mut Selector<'i>) -> Result<(), Self::Error> {
        let mut new_selectors = vec![];
        let mut combinator = None;

        let mut iter = selector.iter();
        loop {
            if combinator.is_none() {
                if let Some(next) = iter.next_sequence() {
                    combinator = Some(next);
                }
            }

            match self.get_transformed_selectors(combinator, &mut iter) {
                Ok(transformed_selectors) => {
                    let new_sel = Selector::from(transformed_selectors.clone());
                    // dbg!(&new_sel);
                    new_selectors.extend(transformed_selectors);
                }
                Err(_) => {
                    // TODO:

                    // HANDLER.with(|handler| {
                    //     handler
                    //         .struct_span_err(
                    //             selector.span,
                    //             "Failed to transform one off global selector",
                    //         )
                    //         .emit()
                    // });

                    new_selectors.extend(iter.clone().cloned());
                }
            }

            if combinator.is_none() {
                if let Some(next) = iter.next_sequence() {
                    combinator = Some(next);
                }
                if combinator.is_none() {
                    break;
                }
            } else {
                combinator = None;
            }
        }

        *selector = Selector::from(new_selectors);

        Ok(())
    }
}

impl Namespacer {
    #[cfg_attr(debug_assertions, tracing::instrument(skip_all))]
    fn get_transformed_selectors<'a, 'i, Impl>(
        &mut self,
        combinator: Option<Combinator>,
        node: &mut SelectorIter<'a, 'i, Impl>,
    ) -> Result<Vec<Component<'i>>, Error>
    where
        Impl: SelectorImpl<'i>,
        SelectorIter<'a, 'i, Impl>: Iterator<Item = &'a Component<'i>>,
    {
        dbg!(&combinator);

        let mut result: Vec<Component<'i>> = vec![];

        let mut pseudo_index = None;

        let mut node = node.fuse();

        for (i, component) in (&mut node).enumerate() {
            trace!("Selector at {}", i);

            // Look for :global
            let children: Selector = match &component {
                Component::NonTSPseudoClass(PseudoClass::CustomFunction { name, arguments }) => {
                    if &**name != "global" {
                        if pseudo_index.is_none() {
                            pseudo_index = Some(i);
                        }

                        result.push(component.clone());
                        continue;
                    }

                    parse_token_list(arguments)
                }
                Component::PseudoElement(_)
                | Component::Negation(..)
                | Component::Root
                | Component::Empty
                | Component::Scope
                | Component::Nth(..)
                | Component::NthOf(..)
                | Component::Slotted(..)
                | Component::Part(..)
                | Component::Host(..)
                | Component::Where(..)
                | Component::Is(..)
                | Component::Any(..)
                | Component::Has(..) => {
                    if pseudo_index.is_none() {
                        pseudo_index = Some(i);
                    }

                    result.push(component.clone());
                    continue;
                }
                _ => {
                    result.push(component.clone());
                    continue;
                }
            };

            let mut complex_selectors = children.iter().cloned().collect::<Vec<_>>();

            // complex_selectors.remove(0);

            if let Component::Combinator(Combinator::Descendant) = complex_selectors[0] {
                complex_selectors.remove(0);
            }

            if complex_selectors.is_empty() {
                bail!("Failed to transform one off global selector");
            }

            trace!("Combinator: {:?}", combinator);
            // trace!("node: {:?}", node);
            // trace!("complex_selectors: {:?}", complex_selectors);

            // result.push(Component::Combinator(Combinator::Descendant));
            result.push(Component::Combinator(Combinator::Descendant));

            if let Some(combinator) = combinator {
                match complex_selectors.get(0) {
                    // `Descendant` combinator can't be the first because we removed it
                    // above
                    Some(Component::Combinator(Combinator::Descendant)) => {}
                    _ => {
                        result.push(Component::Combinator(combinator));
                    }
                }
            }

            result.extend(complex_selectors);
            result.extend(node.cloned());

            return Ok(result);
        }

        if let Some(combinator) = combinator {
            result.push(Component::Combinator(combinator));
        }

        let mut node: Vec<Component<'i>> = node.cloned().collect();

        dbg!(&node);
        dbg!(self.is_global);

        let subclass_selector = match self.is_dynamic {
            true => Cow::Borrowed("__jsx-style-dynamic-selector"),
            false => Cow::Owned(self.class_name.clone()),
        };
        match pseudo_index {
            None => {
                if !self.is_global {
                    node.push(Component::Class(Ident::from(subclass_selector)));
                }

                result.extend(node);
            }

            Some(insert_index) => {
                result.extend(node);

                if !self.is_global {
                    result.insert(
                        insert_index,
                        Component::Class(Ident::from(subclass_selector)),
                    );
                }
            }
        }

        Ok(result)
    }
}

fn parse_token_list<'i>(tokens: &TokenList<'i>) -> Selector<'i> {
    let mut buf = String::new();

    for t in tokens.0.iter() {
        match t {
            TokenOrValue::Token(t) => {
                buf.push_str(&t.to_css_string(Default::default()).unwrap());
            }
            TokenOrValue::Color(t) => {
                buf.push_str(&t.to_css_string(Default::default()).unwrap());
            }
            TokenOrValue::UnresolvedColor(t) => {
                unimplemented!("parse_token_list: unresolved color")
            }
            TokenOrValue::Url(t) => {
                buf.push_str(&t.to_css_string(Default::default()).unwrap());
            }
            TokenOrValue::Var(t) => {
                unimplemented!("parse_token_list: var")
            }
            TokenOrValue::Env(zt) => {
                unimplemented!("parse_token_list: env var")
            }
            TokenOrValue::Function(t) => {
                unimplemented!("parse_token_list: function")
            }
            TokenOrValue::Length(t) => {
                buf.push_str(&t.to_css_string(Default::default()).unwrap());
            }
            TokenOrValue::Angle(t) => {
                buf.push_str(&t.to_css_string(Default::default()).unwrap());
            }
            TokenOrValue::Time(t) => {
                buf.push_str(&t.to_css_string(Default::default()).unwrap());
            }
            TokenOrValue::Resolution(t) => {
                buf.push_str(&t.to_css_string(Default::default()).unwrap());
            }
            TokenOrValue::DashedIdent(t) => {
                buf.push_str(&t.to_css_string(Default::default()).unwrap());
            }
        }
    }
    // TODO: Remove leak
    let selector = Selector::parse_string_with_options(&buf, Default::default())
        .expect("failed to parse selector list");

    owned_selector(&selector)
}

fn owned_selector(sel: &Selector) -> Selector<'static> {
    let mut buf: Vec<Component<'static>> = vec![];

    for component in sel.iter_raw_parse_order_from(0) {
        buf.push(owned_component(component));
    }

    Selector::from(buf)
}

fn owned_component(c: &Component) -> Component<'static> {
    match c {
        parcel_selectors::parser::Component::Combinator(v) => {
            parcel_selectors::parser::Component::Combinator(*v)
        }
        parcel_selectors::parser::Component::ExplicitAnyNamespace => {
            parcel_selectors::parser::Component::ExplicitAnyNamespace
        }
        parcel_selectors::parser::Component::ExplicitNoNamespace => {
            parcel_selectors::parser::Component::ExplicitNoNamespace
        }
        parcel_selectors::parser::Component::DefaultNamespace(v) => {
            parcel_selectors::parser::Component::DefaultNamespace(v.into_owned())
        }
        parcel_selectors::parser::Component::Namespace(v1, v2) => {
            parcel_selectors::parser::Component::Namespace(v1.clone().into_owned(), v2.into_owned())
        }
        parcel_selectors::parser::Component::ExplicitUniversalType => {
            parcel_selectors::parser::Component::ExplicitUniversalType
        }
        parcel_selectors::parser::Component::ID(v) => {
            parcel_selectors::parser::Component::ID(v.into_owned())
        }
        parcel_selectors::parser::Component::Class(v) => {
            parcel_selectors::parser::Component::Class(v.into_owned())
        }
        parcel_selectors::parser::Component::Root => parcel_selectors::parser::Component::Root,
        parcel_selectors::parser::Component::Empty => parcel_selectors::parser::Component::Empty,
        parcel_selectors::parser::Component::Scope => parcel_selectors::parser::Component::Scope,
        parcel_selectors::parser::Component::PseudoElement(v) => {
            parcel_selectors::parser::Component::PseudoElement(*v)
        }
        parcel_selectors::parser::Component::Nesting => {
            parcel_selectors::parser::Component::Nesting
        }

        parcel_selectors::parser::Component::AttributeInNoNamespaceExists {
            local_name,
            local_name_lower,
        } => parcel_selectors::parser::Component::AttributeInNoNamespaceExists {
            local_name: local_name.clone().into_owned(),
            local_name_lower: local_name_lower.clone().into_owned(),
        },
        parcel_selectors::parser::Component::AttributeInNoNamespace {
            local_name,
            operator,
            value,
            case_sensitivity,
            never_matches,
        } => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::AttributeOther(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::Negation(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::LocalName(v1) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::Nth(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::NthOf(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::NonTSPseudoClass(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::Slotted(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::Part(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::Host(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::Where(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::Is(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::Any(v1, v2) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::Has(v) => {
            unimplemented!()
        }
    }
}

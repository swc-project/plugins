use std::{
    borrow::Cow,
    convert::Infallible,
    fmt::{write, Debug},
    mem::transmute,
    panic::{self, catch_unwind, AssertUnwindSafe},
    sync::Arc,
};

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
use parcel_selectors::{
    parser::{LocalName, SelectorIter},
    SelectorImpl,
};
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
    lifetime::owned_selector,
    style::LocalStyle,
    utils::{hash_string, string_literal_expr},
};

#[cfg_attr(
    debug_assertions,
    tracing::instrument(skip(_cm, style_info, class_name))
)]
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
    ss.visit(&mut CssNamespace {
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

struct CssNamespace {
    class_name: String,
    is_global: bool,
    is_dynamic: bool,
}

impl<'i> Visitor<'i> for CssNamespace {
    type Error = Infallible;

    const TYPES: VisitTypes = visit_types!(SELECTORS);

    fn visit_selector(&mut self, selector: &mut Selector<'i>) -> Result<(), Self::Error> {
        let mut new_selectors = vec![];
        let mut combinator = None;

        #[cfg(debug_assertions)]
        let _tracing = tracing::span!(
            tracing::Level::ERROR,
            "visit_selector",
            len = selector.len()
        )
        .entered();

        let mut iter = selector.iter();
        loop {
            if combinator.is_none() {
                combinator = iter.next_sequence();
            }

            match self.get_transformed_selectors(combinator, &mut iter) {
                Ok(transformed_selectors) => {
                    if transformed_selectors.is_empty() {
                        break;
                    }

                    if cfg!(debug_assertions) {
                        let new_sel = Selector::from(transformed_selectors.clone());
                        debug!("Transformed as: {:?}", SafeDebug(&new_sel))
                    }

                    new_selectors.push(transformed_selectors);
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

                    new_selectors.push(iter.clone().cloned().collect());
                }
            }

            if combinator.is_none() {
                combinator = iter.next_sequence();
                if combinator.is_none() {
                    break;
                }
            } else {
                combinator = None;
            }
        }

        let new: Vec<_> = new_selectors.into_iter().rev().flatten().collect();

        *selector = Selector::from(new);

        Ok(())
    }
}

impl CssNamespace {
    #[cfg_attr(debug_assertions, tracing::instrument(skip(self, node)))]
    fn get_transformed_selectors<'a, 'i, Impl>(
        &mut self,
        combinator: Option<Combinator>,
        node: &mut SelectorIter<'a, 'i, Impl>,
    ) -> Result<Vec<Component<'i>>, Error>
    where
        Impl: SelectorImpl<'i>,
        SelectorIter<'a, 'i, Impl>: Iterator<Item = &'a Component<'i>>,
    {
        let mut result: Vec<Component<'i>> = vec![];

        let mut pseudo_index = None;

        let mut node: Vec<Component<'i>> = node.fuse().cloned().collect::<Vec<_>>();

        if node.is_empty() {
            return Ok(result);
        }

        #[cfg(debug_assertions)]
        {
            let prev_sel = Selector::from(node.clone());
            debug!("Input selector: {:?}", SafeDebug(&prev_sel))
        }

        for (i, component) in node.iter().enumerate() {
            trace!("Selector at {}", i);

            #[cfg(debug_assertions)]
            {
                debug!("Component: {:?}", SafeDebug(&component))
            }

            // Look for :global
            let children: Selector<'i> = match &component {
                Component::NonTSPseudoClass(PseudoClass::CustomFunction { name, arguments }) => {
                    if &**name != "global" {
                        if pseudo_index.is_none() {
                            pseudo_index = Some(i);
                        }

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

                    continue;
                }
                _ => {
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
            result.extend(node.into_iter().skip(i + 1));

            return Ok(result);
        }

        // TODO: Combinator for pseudo element
        if result.is_empty() && node.len() == 1 && pseudo_index.is_some() {
            return Ok(node);
        }

        if let Some(combinator) = combinator {
            result.push(Component::Combinator(combinator));
        }

        let mut node: Vec<Component<'i>> = node.clone();

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

    unsafe {
        // Safety: Selector is variant over 'i
        transmute::<Selector, Selector>(owned_selector(&selector))
    }
}

struct SafeDebug<'a>(&'a dyn Debug);

impl Debug for SafeDebug<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = catch_unwind(AssertUnwindSafe(|| format!("{:?}", self.0)));

        match s {
            Ok(s) => {
                write!(f, "{}", s)
            }
            Err(_) => write!(f, "<panicked>"),
        }
    }
}

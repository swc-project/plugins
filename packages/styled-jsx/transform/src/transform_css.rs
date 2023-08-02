use std::{convert::Infallible, panic, sync::Arc};

use easy_error::{bail, Error, ResultExt};
use lightningcss::{
    selector::{Combinator, Component, PseudoClass, Selector},
    stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet},
    traits::ParseWithOptions,
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
        .to_css(PrinterOptions::default())
        .context("failed to print css")?;

    if style_info.expressions.is_empty() {
        return Ok(string_literal_expr(&res.code));
    }

    let mut parts: Vec<&str> = res.code.split("__styled-jsx-placeholder-").collect();
    let mut final_expressions = vec![];
    for i in parts.iter_mut().skip(1) {
        let (num_len, expression_index) = read_number(i);
        final_expressions.push(style_info.expressions[expression_index].clone());
        let substr = &i[(num_len + 2)..];
        *i = substr;
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

    fn visit_selector(&mut self, selector: &mut Selector<'i>) -> Result<(), Self::Error> {
        let mut new_selectors = vec![];
        let mut combinator = None;

        let mut iter = selector.iter();
        loop {
            match self.get_transformed_selectors(combinator, &mut iter) {
                Ok(transformed_selectors) => new_selectors.extend(transformed_selectors),
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

            combinator = None;

            if let Some(next) = iter.next_sequence() {
                combinator = Some(next);
            } else {
                break;
            }
        }

        Ok(())
    }
}

impl Namespacer {
    fn get_transformed_selectors<'a, 'i, Impl>(
        &mut self,
        combinator: Option<Combinator>,
        node: &mut SelectorIter<'a, 'i, Impl>,
    ) -> Result<Vec<Component<'i>>, Error>
    where
        Impl: SelectorImpl<'i>,
        SelectorIter<'a, 'i, Impl>: Iterator<Item = &'a Component<'i>>,
    {
        let mut pseudo_index = None;

        for (i, selector) in node.enumerate() {
            let children = match &selector {
                Component::NonTSPseudoClass(PseudoClass::Global { selector, .. }) => selector,
                Component::NonTSPseudoClass(_) | Component::PseudoElement(_) => {
                    if pseudo_index.is_none() {
                        pseudo_index = Some(i);
                    }

                    continue;
                }
                _ => continue,
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
            trace!("v[0]: {:?}", complex_selectors[0]);

            let mut result = vec![];

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

            // complex_selectors.iter_mut().for_each(|sel| {
            //     if i < node.subclass_selectors.len() {
            //         if let Component::CompoundSelector(sel) = sel {
            //             sel.subclass_selectors
            //                 .extend(node.subclass_selectors[i + 1..].iter().cloned());
            //         }
            //     }
            // });

            result.extend(complex_selectors);

            return Ok(result);
        }

        let mut v = node.cloned().collect::<Vec<Component<'i>>>();

        let subclass_selector = match self.is_dynamic {
            true => "__jsx-style-dynamic-selector",
            false => &self.class_name,
        };
        let insert_index = match pseudo_index {
            None => v.len(),
            Some(i) => i,
        };

        if !self.is_global {
            v.insert(
                insert_index,
                Component::Class(Ident::from(subclass_selector)),
            );
        }

        let mut result = vec![];

        if let Some(combinator) = combinator {
            result.push(Component::Combinator(combinator));
        }

        result.extend(v);

        Ok(result)
    }
}

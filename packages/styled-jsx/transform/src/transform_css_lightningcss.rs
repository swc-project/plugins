use std::{
    borrow::Cow,
    convert::Infallible,
    fmt::Debug,
    mem::transmute,
    panic::{catch_unwind, AssertUnwindSafe},
    sync::{Arc, RwLock},
};

use easy_error::{bail, Error, ResultExt};
use lightningcss::{
    error::ParserError,
    properties::custom::{TokenList, TokenOrValue},
    selector::{Combinator, Component, PseudoClass, Selector},
    stylesheet::{MinifyOptions, ParserFlags, ParserOptions, PrinterOptions, StyleSheet},
    traits::{ParseWithOptions, ToCss},
    values::ident::Ident,
    visit_types,
    visitor::{Visit, VisitTypes, Visitor},
};
use parcel_selectors::{parser::SelectorIter, SelectorImpl};
use swc_common::{
    errors::{DiagnosticBuilder, Level, HANDLER},
    BytePos, FileLines, Loc, SourceMap, Span, DUMMY_SP,
};
use swc_ecma_ast::*;
use tracing::{debug, error, trace};

use crate::{
    lifetime::owned_selector,
    style::LocalStyle,
    utils::{hash_string, string_literal_expr},
};

fn report(
    cm: &SourceMap,
    css_span: Span,
    file_lines_cache: &mut Option<Loc>,
    err: &lightningcss::error::Error<ParserError>,
    level: Level,
) {
    let file = file_lines_cache.get_or_insert_with(|| cm.lookup_char_pos(css_span.lo));

    let lo = if let Some(loc) = &err.loc {
        Some(file.file.lines[(loc.line + 1) as usize] + BytePos(loc.column))
    } else {
        None
    };

    HANDLER.with(|handler| {
        //

        let mut db = DiagnosticBuilder::new(handler, level, &err.kind.to_string());
        if let Some(lo) = lo {
            db.set_span(Span::new(lo, lo, Default::default()));
        }

        db.emit();
    });
}

#[cfg_attr(
    debug_assertions,
    tracing::instrument(skip(cm, style_info, class_name))
)]
pub fn transform_css(
    cm: Arc<SourceMap>,
    style_info: &LocalStyle,
    is_global: bool,
    class_name: &Option<String>,
) -> Result<Expr, Error> {
    let mut file_lines_cache = None;

    debug!("CSS: \n{}", style_info.css);
    let css_str = strip_comments(&style_info.css);

    let warnings: Arc<RwLock<Vec<lightningcss::error::Error<ParserError>>>> = Arc::default();

    let result: Result<StyleSheet, _> = StyleSheet::parse(
        &css_str,
        ParserOptions {
            // We cannot use css_modules for `:global` because lightningcss does not support
            // parsing-only mode.
            css_modules: None,
            error_recovery: true,
            warnings: Some(warnings.clone()),
            flags: ParserFlags::all(),
            ..Default::default()
        },
    );

    let mut ss = match result {
        Ok(ss) => ss,
        Err(err) => {
            HANDLER.with(|handler| {
                report(
                    &cm,
                    style_info.css_span,
                    &mut file_lines_cache,
                    &err,
                    Level::Error,
                );

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

    if let Ok(warnings) = warnings.read() {
        for warning in warnings.iter() {
            report(
                &cm,
                style_info.css_span,
                &mut file_lines_cache,
                warning,
                Level::Warning,
            );
        }
    }

    ss.visit(&mut CssNamespace {
        class_name: match class_name {
            Some(s) => s.clone(),
            None => format!("jsx-{}", &hash_string(&style_info.hash)),
        },
        is_global,
        is_dynamic: style_info.is_dynamic,
    })
    .expect("failed to transform css");

    // Apply auto prefixer
    ss.minify(MinifyOptions {
        ..Default::default()
    })
    .expect("failed to minify/auto-prefix css");

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
        let (num_len, expression_index) = read_number(i, &style_info.is_expr_property);
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

fn strip_comments(s: &str) -> Cow<str> {
    if !s.contains("//") {
        return Cow::Borrowed(s);
    }

    let mut buf = String::with_capacity(s.len());

    for line in s.lines() {
        let line = line.trim();

        if let Some(index) = line.find("//") {
            buf.push_str(&line[..index]);
        } else {
            buf.push_str(line);
        }
        buf.push('\n');
    }

    Cow::Owned(buf)
}

/// Returns `(length, expression_index)`
fn read_number(s: &str, is_expr_property: &[bool]) -> (usize, usize) {
    for (idx, c) in s.char_indices() {
        if c.is_ascii_digit() {
            continue;
        }

        // For 10, we reach here after `0`.
        let expression_index = s[0..idx].parse().expect("failed to parse");

        if is_expr_property[expression_index] {
            return (idx + 3, expression_index);
        }

        return (idx, expression_index);
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

    fn visit_types(&self) -> VisitTypes {
        visit_types!(SELECTORS)
    }

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
            #[cfg(debug_assertions)]
            let _tracing = tracing::span!(
                tracing::Level::ERROR,
                "visit_selector/loop",
                len = iter.selector_length()
            )
            .entered();

            if combinator.is_none() {
                combinator = iter.next_sequence();
            }

            match self.get_transformed_selectors(combinator, &mut iter) {
                Ok(transformed_selectors) => {
                    if transformed_selectors.is_empty() {
                        break;
                    }

                    if cfg!(debug_assertions) {
                        debug!("Transformed as: {:?}", SafeDebug(&transformed_selectors))
                    }

                    new_selectors.push(transformed_selectors);
                }
                Err(_) => {
                    error!("Failed to transform one off global selector");
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

            trace!(
                "Selector length after transform: {}",
                iter.selector_length()
            );

            if combinator.is_none() {
                combinator = iter.next_sequence();
                if combinator.is_none() {
                    break;
                }
            } else {
                combinator = None;
            }
        }

        let new: Vec<_> = RemoveWhitespace {
            iter: new_selectors.into_iter().rev().flatten(),
            prev: None,
        }
        .collect();
        debug!("Selector vector: {:?}", SafeDebug(&new));

        *selector = Selector::from(new);

        Ok(())
    }
}

struct RemoveWhitespace<'i, I> {
    iter: I,
    prev: Option<Component<'i>>,
}

impl<'i, I> Iterator for RemoveWhitespace<'i, I>
where
    I: Iterator<Item = Component<'i>>,
{
    type Item = Component<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.prev.take() {
            Some(Component::Combinator(Combinator::Descendant)) => {
                self.prev = self.iter.next();

                match self.prev {
                    Some(Component::Combinator(..)) => self.next(),
                    _ => Some(Component::Combinator(Combinator::Descendant)),
                }
            }
            Some(v) => Some(v),
            _ => {
                self.prev = self.iter.next();

                if self.prev.is_some() {
                    self.next()
                } else {
                    None
                }
            }
        }
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

        let node: Vec<Component<'i>> = node.fuse().cloned().collect::<Vec<_>>();

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
                | Component::NonTSPseudoClass(..)
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

            let mut complex_selectors =
                children.iter_raw_match_order().cloned().collect::<Vec<_>>();

            // Remove `a`
            complex_selectors.pop();

            if let Some(Component::Combinator(Combinator::Descendant)) = complex_selectors.last() {
                complex_selectors.pop();
            }

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

            complex_selectors.reverse();

            result.extend(complex_selectors);
            result.extend(node.into_iter().skip(i + 1));

            if let Some(combinator) = combinator {
                result.push(Component::Combinator(combinator));
            }

            return Ok(result);
        }

        // Pseudo element
        if result.is_empty() && node.len() == 1 && pseudo_index.is_some() {
            return Ok(node);
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

        if let Some(combinator) = combinator {
            result.push(Component::Combinator(combinator));
        }

        Ok(result)
    }
}

/// Because it is allowed to write `.bar :global(> .foo) {}` or .bar
/// :global(.foo) {}`, so selector can be complex or relative (it violates the
/// specification), but it is popular usage, so we just add `a ` at top and
/// then remove it
fn parse_token_list<'i>(tokens: &TokenList<'i>) -> Selector<'i> {
    let mut buf = "a ".to_string();

    for t in tokens.0.iter() {
        match t {
            TokenOrValue::Token(t) => {
                buf.push_str(&t.to_css_string(Default::default()).unwrap());
            }
            TokenOrValue::Color(t) => {
                buf.push_str(&t.to_css_string(Default::default()).unwrap());
            }
            TokenOrValue::Url(t) => {
                buf.push_str(&t.to_css_string(Default::default()).unwrap());
            }
            TokenOrValue::Var(..) => {
                unimplemented!("parse_token_list: var")
            }
            TokenOrValue::Env(..) => {
                unimplemented!("parse_token_list: env var")
            }
            TokenOrValue::Function(..) => {
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
            TokenOrValue::UnresolvedColor(..) => {
                unimplemented!("parse_token_list: unresolved color")
            }
        }
    }

    if cfg!(debug_assertions) {
        debug!("Parsing: {:?}", buf)
    }

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

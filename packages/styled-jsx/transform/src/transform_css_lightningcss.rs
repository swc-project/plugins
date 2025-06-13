use std::{
    borrow::Cow,
    convert::Infallible,
    fmt::Debug,
    panic::{catch_unwind, AssertUnwindSafe},
    sync::Arc,
};

use anyhow::{bail, Context, Error};
use lightningcss::{
    error::ParserError,
    properties::custom::{TokenList, TokenOrValue},
    selector::{Combinator, Component, PseudoClass, Selector, SelectorList},
    stylesheet::{MinifyOptions, ParserFlags, ParserOptions, PrinterOptions, StyleSheet},
    targets::{Browsers, Features, Targets},
    traits::{IntoOwned, ParseWithOptions, ToCss},
    values::ident::Ident,
    visit_types,
    visitor::{Visit, VisitTypes, Visitor},
};
use parcel_selectors::{parser::SelectorIter, SelectorImpl};
use preset_env_base::{version::Version, Versions};
use swc_common::{
    errors::{DiagnosticBuilder, Level, HANDLER},
    BytePos, Loc, SourceMap, Span, DUMMY_SP,
};
use swc_ecma_ast::*;
use tracing::{debug, error, trace};

use crate::{
    style::LocalStyle,
    utils::{hash_string, string_literal_expr},
    visitor::NativeConfig,
};

fn report(
    cm: &SourceMap,
    css_span: Span,
    file_lines_cache: &mut Option<Loc>,
    err: &lightningcss::error::Error<ParserError>,
    level: Level,
) {
    // We need :global(selector) to be parsed as a selector.
    if let ParserError::SelectorError(
        lightningcss::error::SelectorError::UnsupportedPseudoClass(..)
        | lightningcss::error::SelectorError::UnsupportedPseudoElement(..),
    ) = &err.kind
    {
        return;
    }

    let file = file_lines_cache.get_or_insert_with(|| cm.lookup_char_pos(css_span.lo));

    let lo = if let Some(loc) = &err.loc {
        Some(file.file.analyze().lines[(loc.line + 1) as usize] + BytePos(loc.column))
    } else {
        None
    };

    HANDLER.with(|handler| {
        //

        let mut db = DiagnosticBuilder::new(handler, level, &err.kind.to_string());
        if let Some(lo) = lo {
            db.set_span(Span::new(lo, lo));
        }

        db.emit();
    });
}

#[cfg_attr(
    debug_assertions,
    tracing::instrument(skip(cm, style_info, class_name, browsers, native))
)]
pub fn transform_css(
    cm: Arc<SourceMap>,
    style_info: &LocalStyle,
    is_global: bool,
    class_name: &Option<String>,
    browsers: &Versions,
    native: &NativeConfig,
) -> Result<Expr, Error> {
    let mut file_lines_cache = None;

    let css_str = strip_comments(&style_info.css);

    debug!("CSS: \n{}", css_str);

    let result: Result<StyleSheet, _> = StyleSheet::parse(
        &css_str,
        ParserOptions {
            // We cannot use css_modules for `:global` because lightningcss does not support
            // parsing-only mode.
            css_modules: None,
            error_recovery: true,
            warnings: None,
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

    let targets = Targets {
        browsers: Some(convert_browsers(browsers)),
        include: Features::Nesting,
        ..Default::default()
    };

    // Apply auto prefixer
    ss.minify(MinifyOptions {
        targets: Targets {
            exclude: Features::CustomMediaQueries,
            ..targets
        },
        ..Default::default()
    })
    .expect("failed to minify/auto-prefix css");

    let css = ss
        .to_css(PrinterOptions {
            minify: true,
            targets,
            ..Default::default()
        })
        .context("failed to print css")?;
    let mut ss = {
        StyleSheet::parse(
            &css.code,
            ParserOptions {
                // We cannot use css_modules for `:global` because lightningcss does not support
                // parsing-only mode.
                css_modules: None,
                error_recovery: true,
                warnings: None,
                flags: ParserFlags::all(),
                ..Default::default()
            },
        )
        .unwrap()
    };

    ss.visit(&mut CssNamespace {
        class_name: match class_name {
            Some(s) => s.clone(),
            None => format!("jsx-{}", &hash_string(&style_info.hash)),
        },
        is_global,
        is_dynamic: style_info.is_dynamic,
    })
    .expect("failed to transform css");

    let mut res = ss
        .to_css(PrinterOptions {
            minify: true,
            targets,
            ..Default::default()
        })
        .context("failed to print css")?;

    res.code = native.invoke_css_transform(style_info.css_span, res.code);

    debug!("Transformed CSS: \n{}", res.code);

    if style_info.expressions.is_empty() {
        return Ok(string_literal_expr(&res.code));
    }

    let mut parts: Vec<&str> = res.code.split("--styled-jsx-placeholder-").collect();
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
                    raw: quasi.replace('`', "\\`").replace("\\\"", "\\\\\"").into(),
                    span: DUMMY_SP,
                    tail: false,
                }
            })
            .collect(),
        exprs: final_expressions,
        span: DUMMY_SP,
    }))
}

fn convert_browsers(browsers: &Versions) -> Browsers {
    fn convert(v: Option<Version>) -> Option<u32> {
        v.map(|v| v.major << 16 | v.minor << 8 | v.patch)
    }

    Browsers {
        android: convert(browsers.android),
        chrome: convert(browsers.chrome),
        edge: convert(browsers.edge),
        firefox: convert(browsers.firefox),
        ie: convert(browsers.ie),
        ios_saf: convert(browsers.ios),
        opera: convert(browsers.opera),
        safari: convert(browsers.safari),
        samsung: convert(browsers.samsung),
    }
}

/// Counts occurrences of a character inside string
fn count_occurrences(s: impl AsRef<str>, c: char) -> usize {
    s.as_ref().split(c).count() - 1
}

/// Joins substrings until predicate returns true
fn reduce_substr(
    substrs: impl IntoIterator<Item = impl AsRef<str>>,
    join: &str,
    predicate: impl Fn(&str) -> bool,
) -> String {
    let mut res = "".to_string();

    for (i, substr) in substrs.into_iter().enumerate() {
        if i == 0 {
            res.push_str(substr.as_ref());
            continue;
        }
        if predicate(&res) {
            break;
        }
        res.push_str(join.as_ref());
        res.push_str(substr.as_ref());
    }

    res
}

pub(crate) fn strip_comments(s: &str) -> String {
    s.lines()
        .map(strip_line_comment)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Joins at comment starts when it's inside a string or parentheses
/// effectively removing line comments
fn strip_line_comment(line: &str) -> String {
    reduce_substr(line.split("//"), "//", |s| {
        !s.ends_with(':') // NOTE: This is another guard against urls, if they're not inside strings
            && count_occurrences(s, '\'') % 2 == 0
            && count_occurrences(s, '"') % 2 == 0
    })
}

/// Returns `(length, expression_index)`
pub(super) fn read_number(s: &str, is_expr_property: &[bool]) -> (usize, usize) {
    for (idx, c) in s.char_indices() {
        if c.is_ascii_digit() {
            continue;
        }

        // For 10, we reach here after `0`.
        let expression_index = s[0..idx].parse().expect("failed to parse");

        if is_expr_property[expression_index] {
            return (idx + 2, expression_index);
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
            selector = ?selector
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
            let complex_selectors: Vec<Component<'i>> = match &component {
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

            #[cfg(debug_assertions)]
            debug!("Complex selectors: {:?}", complex_selectors);

            result.extend(complex_selectors);
            result.extend(node.into_iter().skip(i + 1));

            if let Some(combinator) = combinator {
                result.push(Component::Combinator(combinator));
            }

            return Ok(result);
        }

        // Pseudo element
        if result.is_empty()
            && node.len() == 1
            && pseudo_index.is_some()
            && matches!(&node[0], Component::PseudoElement(..))
        {
            return Ok(node);
        }

        let mut node: Vec<Component<'i>> = node.clone();

        if result.is_empty()
            && node.len() == 1
            && matches!(node[0], Component::Nesting)
            && matches!(combinator, Some(Combinator::Descendant))
        {
            node.push(Component::Combinator(Combinator::Descendant));
            return Ok(node);
        }

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
fn parse_token_list<'i>(tokens: &TokenList<'i>) -> Vec<Component<'i>> {
    let mut buf = "".to_string();

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
            TokenOrValue::AnimationName(_) => {
                unimplemented!("parse_token_list: animation name")
            }
        }
    }

    if cfg!(debug_assertions) {
        debug!("Parsing: {:?}", buf)
    }

    if let Ok(s) = SelectorList::parse_string_with_options(&buf, Default::default()) {
        if s.0.len() != 1 {
            return vec![Component::Is(s.0.into_owned().into_boxed_slice())];
        }
    }

    buf = format!("a {buf}");

    let mut result: Vec<Component<'i>> = vec![];

    let selector = Selector::parse_string_with_options(&buf, Default::default())
        .expect("failed to parse selector list")
        .into_owned();

    // Compound selectors invert the order of their contents, so we need to
    // undo that during serialization.
    //
    // This two-iterator strategy involves walking over the selector twice.
    // We could do something more clever, but selector serialization probably
    // isn't hot enough to justify it, and the stringification likely
    // dominates anyway.
    //
    // NB: A parse-order iterator is a Rev<>, which doesn't expose as_slice(),
    // which we need for |split|. So we split by combinators on a match-order
    // sequence and then reverse.

    let mut combinators = selector
        .iter_raw_match_order()
        .rev()
        .filter_map(|x| x.as_combinator());
    let compound_selectors = selector
        .iter_raw_match_order()
        .as_slice()
        .split(|x| x.is_combinator())
        .rev();

    let mut combinators_exhausted = false;
    for compound in compound_selectors {
        debug_assert!(!combinators_exhausted);

        // https://drafts.csswg.org/cssom/#serializing-selectors
        if compound.is_empty() {
            continue;
        }

        // 1. If there is only one simple selector in the compound selectors which is a
        //    universal selector, append the result of serializing the universal
        //    selector to s.
        //
        // Check if `!compound.empty()` first--this can happen if we have
        // something like `... > ::before`, because we store `>` and `::`
        // both as combinators internally.
        //
        // If we are in this case, after we have serialized the universal
        // selector, we skip Step 2 and continue with the algorithm.
        let (can_elide_namespace, first_non_namespace) = match compound[0] {
            Component::ExplicitAnyNamespace
            | Component::ExplicitNoNamespace
            | Component::Namespace(..) => (false, 1),
            Component::DefaultNamespace(..) => (true, 1),
            _ => (true, 0),
        };
        let mut perform_step_2 = true;
        let next_combinator = combinators.next();
        if first_non_namespace == compound.len() - 1 {
            match (next_combinator, &compound[first_non_namespace]) {
                // We have to be careful here, because if there is a
                // pseudo element "combinator" there isn't really just
                // the one simple selector. Technically this compound
                // selector contains the pseudo element selector as well
                // -- Combinator::PseudoElement, just like
                // Combinator::SlotAssignment, don't exist in the
                // spec.
                (Some(Combinator::PseudoElement), _) | (Some(Combinator::SlotAssignment), _) => (),
                (_, &Component::ExplicitUniversalType) => {
                    // Iterate over everything so we serialize the namespace
                    // too.
                    for simple in compound.iter() {
                        result.push(simple.clone());
                    }
                    // Skip step 2, which is an "otherwise".
                    perform_step_2 = false;
                }
                _ => (),
            }
        }

        // 2. Otherwise, for each simple selector in the compound selectors that is not
        //    a universal selector of which the namespace prefix maps to a namespace
        //    that is not the default namespace serialize the simple selector and append
        //    the result to s.
        //
        // See https://github.com/w3c/csswg-drafts/issues/1606, which is
        // proposing to change this to match up with the behavior asserted
        // in cssom/serialize-namespaced-type-selectors.html, which the
        // following code tries to match.
        if perform_step_2 {
            for simple in compound.iter() {
                if let Component::ExplicitUniversalType = *simple {
                    // Can't have a namespace followed by a pseudo-element
                    // selector followed by a universal selector in the same
                    // compound selector, so we don't have to worry about the
                    // real namespace being in a different `compound`.
                    if can_elide_namespace {
                        continue;
                    }
                }
                result.push(simple.clone());
            }
        }

        // 3. If this is not the last part of the chain of the selector append a single
        //    SPACE (U+0020), followed by the combinator ">", "+", "~", ">>", "||", as
        //    appropriate, followed by another single SPACE (U+0020) if the combinator
        //    was not whitespace, to s.
        match next_combinator {
            Some(c) => result.push(Component::Combinator(c)),
            None => combinators_exhausted = true,
        };

        // 4. If this is the last part of the chain of the selector and there is
        //    a pseudo-element, append "::" followed by the name of the
        //    pseudo-element, to s.
        //
        // (we handle this above)
    }

    // Remove `a`
    result.remove(0);

    if let Some(Component::Combinator(Combinator::Descendant)) = result.first() {
        result.remove(0);
    }

    if let Some(Component::Combinator(Combinator::Descendant)) = result.last() {
        result.pop();
    }

    result
}

struct SafeDebug<'a>(&'a dyn Debug);

impl Debug for SafeDebug<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = catch_unwind(AssertUnwindSafe(|| format!("{:?}", self.0)));

        match s {
            Ok(s) => {
                write!(f, "{s}")
            }
            Err(_) => write!(f, "<panicked>"),
        }
    }
}

use std::{panic, sync::Arc};

use easy_error::{bail, Error, ResultExt};
use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
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

    // TODO use `parse_string_input` in future
    let config = ParserConfig {
        allow_wrong_line_comments: true,
        css_modules: false,
        ..Default::default()
    };

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
                err.to_diagnostics(handler).emit();

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
    // ? Do we need to support optionally prefixing?
    ss.visit_mut_with(&mut prefixer(Default::default()));
    ss.visit_mut_with(&mut Namespacer {
        class_name: match class_name {
            Some(s) => s.clone(),
            None => format!("jsx-{}", &hash_string(&style_info.hash)),
        },
        is_global,
        is_dynamic: style_info.is_dynamic,
    });

    let res = ss
        .to_css(PrinterOptions::default())
        .context("failed to print css")?;

    if style_info.expressions.is_empty() {
        return Ok(string_literal_expr(&res));
    }

    let mut parts: Vec<&str> = res.split("__styled-jsx-placeholder-").collect();
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

impl VisitMut for Namespacer {
    fn visit_mut_complex_selector(&mut self, node: &mut ComplexSelector) {
        #[cfg(debug_assertions)]
        let _tracing = {
            // This will add information to the log messages, only for debug build.
            // Note that we use cargo feature to remove all logging on production builds.

            let mut code = String::new();
            {
                let mut wr = BasicCssWriter::new(&mut code, None, BasicCssWriterConfig::default());
                let mut gen = CodeGenerator::new(&mut wr, CodegenConfig { minify: true });

                gen.emit(&*node).unwrap();
            }

            tracing::span!(
                tracing::Level::TRACE,
                "Namespacer::visit_mut_complex_selector",
                class_name = &*self.class_name,
                is_global = self.is_global,
                is_dynamic = self.is_dynamic,
                input = &*code
            )
            .entered()
        };

        let mut new_selectors = vec![];
        let mut combinator = None;

        for sel in node.children.take() {
            match &sel {
                ComplexSelectorChildren::CompoundSelector(selector) => {
                    match self.get_transformed_selectors(combinator, selector.clone()) {
                        Ok(transformed_selectors) => new_selectors.extend(transformed_selectors),
                        Err(_) => {
                            HANDLER.with(|handler| {
                                handler
                                    .struct_span_err(
                                        selector.span,
                                        "Failed to transform one off global selector",
                                    )
                                    .emit()
                            });
                            new_selectors.push(sel);
                        }
                    }

                    combinator = None;
                }
                ComplexSelectorChildren::Combinator(v) => {
                    combinator = Some(v.clone());
                }
            };
        }

        node.children = new_selectors;
    }
}

impl Namespacer {
    fn get_transformed_selectors(
        &mut self,
        combinator: Option<Combinator>,
        mut node: CompoundSelector,
    ) -> Result<Vec<ComplexSelectorChildren>, Error> {
        let mut pseudo_index = None;

        for (i, selector) in node.subclass_selectors.iter().enumerate() {
            let (name, children) = match &selector {
                SubclassSelector::PseudoClass(PseudoClassSelector {
                    name,
                    children: Some(children),
                    ..
                }) if &name.value == "global" => (name, children),
                SubclassSelector::PseudoClass(_) | SubclassSelector::PseudoElement(_) => {
                    if pseudo_index.is_none() {
                        pseudo_index = Some(i);
                    }

                    continue;
                }
                _ => continue,
            };

            // One off global selector
            if &name.value == "global" {
                // TODO(alexander-akait): in future we should use list of component values
                let tokens = children
                    .iter()
                    .map(|v| match v {
                        PseudoClassSelectorChildren::PreservedToken(v) => v.clone(),
                        _ => {
                            unreachable!();
                        }
                    })
                    .collect::<Vec<TokenAndSpan>>();
                let mut tokens = {
                    let lo = tokens.first().map(|v| v.span_lo()).unwrap_or(BytePos(0));
                    let hi = tokens.last().map(|v| v.span_hi()).unwrap_or(BytePos(0));

                    Tokens {
                        span: Span::new(lo, hi, Default::default()),
                        tokens,
                    }
                };

                // Because it is allowed to write `.bar :global(> .foo) {}` or .bar
                // :global(.foo) {}`, so selector can be complex or relative (it violates the
                // specification), but it is popular usage, so we just add `a >` at top and then
                // remove it
                let mut front_tokens = get_front_selector_tokens(&tokens);

                front_tokens.extend(tokens.tokens);

                tokens.tokens = front_tokens;

                let complex_selectors = panic::catch_unwind(|| {
                    let x: ComplexSelector = parse_input(
                        InputType::Tokens(&tokens),
                        ParserConfig {
                            allow_wrong_line_comments: true,
                            css_modules: true,
                            ..Default::default()
                        },
                        // TODO(kdy1): We might be able to report syntax errors.
                        &mut vec![],
                    )
                    .unwrap();
                    x
                });

                return match complex_selectors {
                    Ok(complex_selectors) => {
                        let mut v = complex_selectors.children[1..].to_vec();

                        if let ComplexSelectorChildren::Combinator(Combinator {
                            value: CombinatorValue::Descendant,
                            ..
                        }) = v[0]
                        {
                            v.remove(0);
                        }

                        if v.is_empty() {
                            bail!("Failed to transform one off global selector");
                        }

                        trace!("Combinator: {:?}", combinator);
                        trace!("v[0]: {:?}", v[0]);

                        let mut result = vec![];

                        if let Some(combinator) = combinator {
                            match v.get(0) {
                                // `Descendant` combinator can't be the first because we removed it
                                // above
                                Some(ComplexSelectorChildren::Combinator(..))
                                    if combinator.value == CombinatorValue::Descendant => {}
                                _ => {
                                    result.push(ComplexSelectorChildren::Combinator(combinator));
                                }
                            }
                        }

                        v.iter_mut().for_each(|sel| {
                            if i < node.subclass_selectors.len() {
                                if let ComplexSelectorChildren::CompoundSelector(sel) = sel {
                                    sel.subclass_selectors
                                        .extend(node.subclass_selectors[i + 1..].iter().cloned());
                                }
                            }
                        });

                        result.extend(v);

                        Ok(result)
                    }
                    Err(_) => bail!("Failed to transform one off global selector"),
                };
            }
        }

        let subclass_selector = match self.is_dynamic {
            true => "__jsx-style-dynamic-selector",
            false => &self.class_name,
        };
        let insert_index = match pseudo_index {
            None => node.subclass_selectors.len(),
            Some(i) => i,
        };

        if !self.is_global {
            node.subclass_selectors.insert(
                insert_index,
                SubclassSelector::Class(ClassSelector {
                    span: DUMMY_SP,
                    text: Ident {
                        raw: Some(subclass_selector.into()),
                        value: subclass_selector.into(),
                        span: DUMMY_SP,
                    },
                }),
            );
        }

        let mut result = vec![];

        if let Some(combinator) = combinator {
            result.push(ComplexSelectorChildren::Combinator(combinator));
        }

        result.push(ComplexSelectorChildren::CompoundSelector(node));

        Ok(result)
    }
}

fn get_front_selector_tokens(selector_tokens: &Tokens) -> Vec<TokenAndSpan> {
    let start_pos = selector_tokens.span.lo.to_u32() - 2;
    vec![
        TokenAndSpan {
            span: Span {
                lo: BytePos(start_pos),
                hi: BytePos(start_pos + 1),
                ctxt: SyntaxContext::empty(),
            },
            token: Token::Ident {
                raw: "a".into(),
                value: "a".into(),
            },
        },
        TokenAndSpan {
            span: Span {
                lo: BytePos(start_pos + 1),
                hi: BytePos(start_pos + 2),
                ctxt: SyntaxContext::empty(),
            },
            token: Token::WhiteSpace { value: " ".into() },
        },
    ]
}

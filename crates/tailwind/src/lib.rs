#![feature(box_patterns)]

use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, Result};
#[cfg(feature = "parallel")]
use rayon::prelude::*;
use regex::Regex;
use swc_common::{
    collections::{AHashMap, AHashSet},
    errors::HANDLER,
    sync::Lazy,
    util::take::Take,
    FileName, SourceMap, DUMMY_SP,
};
use swc_core::{
    css::{
        ast::{
            AtRule, AtRuleName, AtRulePrelude, ComponentValue, Declaration, DeclarationName, Ident,
            ListOfComponentValues, QualifiedRule, QualifiedRulePrelude, Rule, SimpleBlock,
            Stylesheet, Token, TokenAndSpan,
        },
        parser::parse_file,
        visit::{VisitMut, VisitMutWith},
    },
    ecma::atoms::js_word,
};

/// Content of the config file
#[derive(Debug)]
pub struct Config {
    content: Vec<String>,
}

impl Config {
    pub fn from_path(path: &Path) -> Result<Self> {
        todo!()
    }
}

static CONTENT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"['"\s<>=/]"#).unwrap());

pub struct Tailwind {
    cm: Arc<SourceMap>,
    config_path: PathBuf,
}

impl Tailwind {
    pub fn new(cm: Arc<SourceMap>, config_path: PathBuf) -> Self {
        Self { config_path, cm }
    }

    pub fn compile(&mut self, ss: &mut Stylesheet) -> Result<()> {
        let config = Config::from_path(&self.config_path)
            .context("failed to load config file")
            .map(Arc::new)?;

        let files = resolve_glob(&config.content);

        // Collect candidates, optionally in parallel
        #[cfg(not(feature = "parallel"))]
        let iter = files.into_iter();

        #[cfg(feature = "parallel")]
        let iter = files.into_par_iter();

        // TODO: Optimize collect out
        let candidates = iter
            .map(|f| {
                let contents = read_to_string(&f).context("failed to read file")?;

                let s = CONTENT_RE
                    .split(&contents)
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();

                Ok(s)
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<AHashSet<_>>();

        let mut plugins: Vec<Plugin> = vec![];

        // Built-in plugins
        plugins.push(Box::new(|context| {
            let mut map = AHashMap::default();

            map.insert(".built-in-utility".into(), {
                let mut m = AHashMap::default();
                m.insert("color".into(), "red".into());
                m
            });
            map.insert(".should-not-be-generated".into(), {
                let mut m = AHashMap::default();
                m.insert("appearance".into(), "none".into());
                m
            });

            context.add_utilities(map);
        }));

        // Example built-in plugin that can read values from the config.

        plugins.push({
            let config = config.clone();
            Box::new(move |context| {
                let map = AHashMap::default();

                // TODO: Convert config to a hash map

                // This is an example of using config from core plugins
                #[allow(clippy::drop_ref)]
                drop(&config);
                context.add_utilities(map);
            })
        });

        // External plugins registered in the `tailwind.config.js` file.

        // This requires a trick.
        // Note: We can invoke js function from standalone node addon, but we can't from
        // wasm plugin.

        // TODO:
        // plugins.extend(config.plugins);

        // Collect "plugins" from the CSS
        //
        // NOTE: In reality we want to collect information for the correct layer. But
        // for this proof of concept that does not matter. Idea is that we _can_
        // read the CSS file and collect information from it.

        ss.visit_mut_with(&mut PluginCollector {
            plugins: &mut plugins,
        });

        // Generate all of the CSS by looking at the classes extracted from the template
        // files registered in the user's `content` configuration and matching
        // them with the plugins we registered with Tailwind above.

        let mut new_rules = vec![];

        for plugin in plugins {
            plugin(&mut PluginContext {
                cm: &self.cm,
                candidates: &candidates,
                new_rules: &mut new_rules,
            });
        }

        // Replace the @tailwind rule with the CSS that was generated based on the
        // user's template contents.

        ss.visit_mut_with(&mut TailwindReplacer {
            new_rules: &mut new_rules,
            extra: Default::default(),
        });

        Ok(())
    }
}

fn resolve_glob(config: &[String]) -> Vec<PathBuf> {
    todo!()
}

type Plugin = Box<dyn for<'aa> Fn(&mut PluginContext<'aa>)>;

pub struct PluginContext<'a> {
    cm: &'a SourceMap,
    candidates: &'a AHashSet<String>,
    new_rules: &'a mut Vec<Rule>,
}

impl PluginContext<'_> {
    /// `map`: `(selector, definitions)`
    pub fn add_utilities(&mut self, map: AHashMap<String, AHashMap<String, String>>) {
        // Only generate the rules that we care about.
        // .slice(1) is a quick way of getting rid of the `.` of the selector
        // Very naive, but as a proof-of-concept this is fine.
        for (selector, definitions) in map {
            if self.candidates.contains(&selector[1..]) {
                // TODO: Customize FileName so that we can generate correct source map
                //
                // The source map system of swc is designed to allow pointing different files.

                macro_rules! parse {
                    ($input:expr) => {{
                        let mut errors = vec![];

                        let fm = self.cm.new_source_file(FileName::Anon, $input);
                        let n = parse_file(&fm, Default::default(), &mut errors);

                        // Report errors
                        for err in errors {
                            HANDLER.with(|handler| {
                                err.to_diagnostics(handler).emit();
                            });
                        }

                        match n {
                            Ok(v) => Some(v),
                            Err(err) => {
                                HANDLER.with(|handler| {
                                    err.to_diagnostics(handler).emit();
                                });
                                None
                            }
                        }
                    }};
                }

                let prelude = parse!(selector).map(QualifiedRulePrelude::SelectorList);

                let body = definitions
                    .into_iter()
                    .filter_map(|(property, value)| {
                        let name = if property.contains("-") {
                            parse!(property).map(DeclarationName::DashedIdent)
                        } else {
                            parse!(property).map(DeclarationName::Ident)
                        }?;
                        let value: ListOfComponentValues = parse!(value)?;

                        Some(ComponentValue::Declaration(Declaration {
                            span: DUMMY_SP,
                            name,
                            value: value.children,
                            important: Default::default(),
                        }))
                    })
                    .collect();

                let prelude = match prelude {
                    Some(v) => v,
                    None => continue,
                };

                self.new_rules
                    .push(Rule::QualifiedRule(Box::new(QualifiedRule {
                        span: DUMMY_SP,
                        prelude,
                        block: SimpleBlock {
                            span: DUMMY_SP,
                            name: TokenAndSpan {
                                span: DUMMY_SP,
                                token: Token::LBrace,
                            },
                            value: body,
                        },
                    })));
            }
        }
    }
}

struct PluginCollector<'a> {
    plugins: &'a mut Vec<Plugin>,
}

impl VisitMut for PluginCollector<'_> {
    fn visit_mut_rules(&mut self, n: &mut Vec<Rule>) {
        n.visit_mut_children_with(self);

        n.retain(|r| match r {
            Rule::AtRule(r) => {
                if let AtRuleName::Ident(name) = &r.name {
                    name.value != js_word!("")
                } else {
                    true
                }
            }

            _ => true,
        })
    }

    fn visit_mut_at_rule(&mut self, n: &mut AtRule) {
        if let AtRuleName::Ident(name) = &n.name {
            if &*name.value == "layer" {
                if let Some(box AtRulePrelude::ListOfComponentValues(tokens)) = &n.prelude {
                    for v in &tokens.children {
                        if let Some(selector_name) = v
                            .as_preserved_token()
                            .map(|v| &v.token)
                            .and_then(extract_directive)
                        {
                            // @layer utilities {}

                            let mut collector = DeclCollector::default();
                            n.block.visit_mut_with(&mut collector);

                            self.plugins.push(Box::new(|context| {
                                let mut m = AHashMap::default();

                                // TODO: Add collector.decls to m
                                context.add_utilities(m);
                            }));

                            // Remove @layer
                            n.name = AtRuleName::Ident(Ident {
                                span: DUMMY_SP,
                                value: js_word!(""),
                                raw: None,
                            });
                            return;
                        }
                    }
                }
            }
        }

        n.visit_mut_children_with(self);

        //
    }
}

fn extract_directive(t: &Token) -> Option<&str> {
    if let Token::Ident { value, .. } = t {
        Some(&**value)
    } else {
        None
    }
}

#[derive(Default)]
struct DeclCollector {
    decls: Vec<Declaration>,
}

impl VisitMut for DeclCollector {
    fn visit_mut_declaration(&mut self, n: &mut Declaration) {
        // TODO: Remove clone using mem::replace
        self.decls.push(n.clone());
    }
}

struct TailwindReplacer<'a> {
    new_rules: &'a mut Vec<Rule>,
    extra: Vec<Rule>,
}

impl VisitMut for TailwindReplacer<'_> {
    fn visit_mut_rules(&mut self, n: &mut Vec<Rule>) {
        let prev = self.extra.take();

        let mut new = Vec::with_capacity(n.len() + 4);

        for mut r in n.take() {
            r.visit_mut_with(self);

            new.extend(self.extra.take());

            if let Rule::AtRule(r) = &r {
                if let AtRuleName::Ident(name) = &r.name {
                    if name.value == js_word!("") {
                        continue;
                    }
                }
            }
            new.push(r);
        }

        *n = new;
        self.extra = prev;
    }

    fn visit_mut_at_rule(&mut self, n: &mut AtRule) {
        n.visit_mut_children_with(self);

        if let AtRuleName::Ident(name) = &n.name {
            if &*name.value == "tailwind" {
                if let Some(box AtRulePrelude::ListOfComponentValues(tokens)) = &n.prelude {
                    // Remove @tailwind
                    n.name = AtRuleName::Ident(Ident {
                        span: DUMMY_SP,
                        value: js_word!(""),
                        raw: None,
                    });
                    self.extra.extend(self.new_rules.take());
                }
            }
        }

        //
    }
}

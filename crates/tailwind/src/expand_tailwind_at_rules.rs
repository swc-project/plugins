use indexmap::IndexSet;
use swc_atoms::js_word;
use swc_core::{
    common::{collections::AHashSet, util::take::Take, DUMMY_SP},
    css::{
        ast::{AtRule, AtRuleName, AtRulePrelude, Ident, Rule, Stylesheet, Token},
        visit::{Visit, VisitMut, VisitMutWith, VisitWith},
    },
};
use swc_timer::timer;

use crate::{
    base::{Candidate, LayerNode},
    context::Context,
    generate_rules::generate_rules,
};

pub(crate) fn expand_tailwind_at_rules(context: &mut Context, ss: &mut Stylesheet) {
    let mut layers = AHashSet::<LayerNode>::default();

    ss.visit_with(&mut TailwindFinder {
        layers: &mut layers,
    });

    if layers.is_empty() {
        return;
    }

    // Find potential rules in changed files
    let mut candidates = IndexSet::default();
    // let mut seen = AHashSet::default();

    candidates.insert(Candidate::NotOnDemand);

    // TODO(kdy1): Port

    {
        let _timer = timer!("Reading changed files");

        // for (let { content, extension } of context.changedContent) {
        //     let transformer = getTransformer(context.tailwindConfig,
        // extension)     let extractor = getExtractor(context,
        // extension)     getClassCandidates(transformer(content),
        // extractor, candidates, seen) }
    }
    //

    {
        let _timer = timer!("Generate rules");
        generate_rules(&candidates, context);
    }

    let mut new_stylesheet = {
        let _timer = timer!("Build stylesheet");
        build_stylesheet(context)
    };

    // Replace any Tailwind directives with generated CSS
    ss.visit_mut_with(&mut TailwindReplacer {
        built: &mut new_stylesheet,
        extra: Default::default(),
    });

    // Cleanup any leftover @layer at-rules
    ss.visit_mut_with(&mut LayerRemover);
}

fn build_stylesheet(context: &mut Context) -> BuiltStylesheet {
    Default::default()
}

#[derive(Debug, Default)]
struct BuiltStylesheet {
    base: AHashSet<Rule>,
    defaults: AHashSet<Rule>,
    components: AHashSet<Rule>,
    utilities: AHashSet<Rule>,
    variants: AHashSet<Rule>,
}

/// This removes `@tailwind` directives.
struct TailwindReplacer<'a> {
    built: &'a mut BuiltStylesheet,
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
                    for v in &tokens.children {
                        if let Some(v) = v
                            .as_preserved_token()
                            .map(|v| &v.token)
                            .and_then(extract_directive)
                        {
                            match v {
                                "base" => {
                                    self.extra.extend(self.built.base.drain());
                                    self.extra.extend(self.built.defaults.iter().cloned());
                                }
                                "components" => {
                                    self.extra.extend(self.built.components.drain());
                                    self.extra.extend(self.built.defaults.iter().cloned());
                                }
                                "utilities" => {
                                    self.extra.extend(self.built.utilities.drain());
                                    self.extra.extend(self.built.defaults.iter().cloned());
                                }
                                "variants" => {
                                    // TODO
                                }
                                _ => continue,
                            }

                            n.name = AtRuleName::Ident(Ident {
                                span: DUMMY_SP,
                                value: js_word!(""),
                                raw: None,
                            });
                        }
                    }
                }
            }
        }

        //
    }
}

struct TailwindFinder<'a> {
    layers: &'a mut AHashSet<LayerNode>,
}

impl Visit for TailwindFinder<'_> {
    fn visit_at_rule(&mut self, n: &AtRule) {
        n.visit_children_with(self);

        if let AtRuleName::Ident(name) = &n.name {
            if &*name.value == "tailwind" {
                if let Some(box AtRulePrelude::ListOfComponentValues(tokens)) = &n.prelude {
                    for v in &tokens.children {
                        if let Some(v) = v
                            .as_preserved_token()
                            .map(|v| &v.token)
                            .and_then(extract_directive)
                        {
                            match v {
                                "base" => {
                                    self.layers.insert(LayerNode::Base);
                                }
                                "components" => {
                                    self.layers.insert(LayerNode::Components);
                                }
                                "utilities" => {
                                    self.layers.insert(LayerNode::Utilities);
                                }
                                "variants" => {
                                    self.layers.insert(LayerNode::Variant);
                                }
                                _ => continue,
                            }
                        }
                    }
                }
            }
        }

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

struct LayerRemover;

impl VisitMut for LayerRemover {
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
        n.visit_mut_children_with(self);

        if let AtRuleName::Ident(name) = &n.name {
            if &*name.value == "layer" {
                if let Some(box AtRulePrelude::ListOfComponentValues(tokens)) = &n.prelude {
                    for v in &tokens.children {
                        if let Some(v) = v
                            .as_preserved_token()
                            .map(|v| &v.token)
                            .and_then(extract_directive)
                        {
                            match v {
                                "base" | "components" | "utilities" | "variants" => {
                                    n.name = AtRuleName::Ident(Ident {
                                        span: DUMMY_SP,
                                        value: js_word!(""),
                                        raw: None,
                                    });
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        //
    }
}

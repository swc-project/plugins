use swc_atoms::js_word;
use swc_core::{
    common::{collections::AHashSet, DUMMY_SP},
    css::{
        ast::{
            AtRule, AtRuleName, AtRulePrelude, ComponentValue, Ident, Rule, Stylesheet, Token,
            TokenAndSpan,
        },
        visit::{VisitMut, VisitMutWith},
    },
};
use swc_timer::timer;

use crate::context::Context;

pub(crate) fn expand_tailwind_at_rules(context: &mut Context, ss: &mut Stylesheet) {
    let mut layers = AHashSet::<LayerNode>::default();

    ss.visit_mut_with(&mut TailwindFinder {
        layers: &mut layers,
    });

    if layers.is_empty() {
        return;
    }

    // Find potential rules in changed files
    let mut candidates = AHashSet::default();
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

    let new_stylesheet = {
        let _timer = timer!("Build stylesheet");
        build_stylesheet(context)
    };

    // Cleanup any leftover @layer at-rules
    ss.visit_mut_with(&mut LayerRemover);
}

fn build_stylesheet(context: &mut Context) -> BuiltStylesheet {
    Default::default()
}

fn generate_rules(candidates: &AHashSet<Candidate>, context: &mut Context) {}

#[derive(Debug, Default)]
struct BuiltStylesheet {
    base: AHashSet<Rule>,
    defaults: AHashSet<Rule>,
    components: AHashSet<Rule>,
    utilities: AHashSet<Rule>,
    variants: AHashSet<Rule>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Candidate {
    NotOnDemand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum LayerNode {
    Base,
    Components,
    Utilities,
    Variant,
}

/// This removes `@tailwind` directives.
struct TailwindFinder<'a> {
    layers: &'a mut AHashSet<LayerNode>,
}

impl VisitMut for TailwindFinder<'_> {
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

use swc_atoms::{atom, js_word};
use swc_core::{
    common::{collections::AHashSet, DUMMY_SP},
    css::{
        ast::{AtRule, AtRuleName, ComponentValue, Ident, Rule, Stylesheet},
        visit::{Visit, VisitMut, VisitMutWith, VisitWith},
    },
};
use swc_timer::timer;

use crate::context::Context;

pub(crate) fn expand_tailwind_at_rules(context: &mut Context, ss: &mut Stylesheet) {
    let mut layers = AHashSet::<LayerNode>::default();

    ss.visit_with(&mut TailwindFinder {
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

struct TailwindFinder<'a> {
    layers: &'a mut AHashSet<LayerNode>,
}

impl Visit for TailwindFinder<'_> {
    fn visit_at_rule(&mut self, n: &AtRule) {
        n.visit_children_with(self);

        if let AtRuleName::Ident(name) = &n.name {
            if &*name.value == "tailwind" {
                if let Some(block) = &n.block {
                    for v in &block.value {
                        if let ComponentValue::Ident(i) = v {
                            match &*i.value {
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

struct LayerRemover;

impl VisitMut for LayerRemover {
    fn visit_mut_rules(&mut self, n: &mut Vec<Rule>) {
        n.visit_mut_children_with(self);

        n.retain(|r| match r {
            Rule::AtRule(r) => {
                if let AtRuleName::Ident(name) = &r.name {
                    &*name.value != ""
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
                if let Some(block) = &n.block {
                    for v in &block.value {
                        if let ComponentValue::Ident(i) = v {
                            match &*i.value {
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

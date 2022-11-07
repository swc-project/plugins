use swc_core::{
    common::collections::AHashSet,
    css::{
        ast::{AtRule, AtRuleName, ComponentValue, Stylesheet},
        visit::{Visit, VisitWith},
    },
};

pub(crate) fn expand_tailwind_at_rules(ss: &mut Stylesheet) {
    let mut layers = AHashSet::<LayerNode>::default();

    ss.visit_with(&mut TailwindFinder {
        layers: &mut layers,
    });

    if layers.is_empty() {
        return;
    }

    //
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayerNode {
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

use swc_core::{
    common::collections::AHashSet,
    css::{
        ast::{AtRule, AtRuleName, Stylesheet},
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

        match &n.name {
            AtRuleName::Ident(n) => {
                if &*n.value != "tailwind" {
                    return;
                }
            }
            _ => return,
        }

        //
    }
}

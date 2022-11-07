use swc_core::{
    common::util::take::Take,
    css::{
        ast::{
            AtRule, AtRuleName, AtRulePrelude, ComponentValue, DeclarationOrAtRule, Rule,
            Stylesheet,
        },
        visit::{VisitMut, VisitMutWith},
    },
};

pub fn partition_apply_at_rules(ss: &mut Stylesheet) {
    ss.visit_mut_with(&mut Visitor::default());
}

#[derive(Debug, Default)]
struct Visitor {}

impl VisitMut for Visitor {
    fn visit_mut_at_rule(&mut self, n: &mut AtRule) {
        n.visit_mut_children_with(self);

        if let AtRuleName::Ident(name) = &n.name {
            // We are interested in only this node
            if &*name.value == "apply" {
                match &mut n.block {
                    Some(body) => {
                        let mut node_groups = vec![];
                        let mut last_group = vec![];

                        for item in body.value.take() {
                            match item {
                                ComponentValue::DeclarationOrAtRule(
                                    DeclarationOrAtRule::AtRule(rule),
                                )
                                | ComponentValue::Rule(Rule::AtRule(rule)) => {
                                    if !last_group.is_empty() {
                                        node_groups.push(last_group);
                                        last_group = vec![];
                                    }
                                    node_groups
                                        .push(vec![ComponentValue::Rule(Rule::AtRule(rule))]);
                                }

                                _ => {
                                    last_group.push(item);
                                }
                            }
                        }

                        if !last_group.is_empty() {
                            node_groups.push(last_group);
                        }

                        if node_groups.len() == 1 {
                            // We don't need to partition
                            return;
                        }

                        for nodes in node_groups.into_iter().rev() {}
                    }
                    None => {}
                }
            }
        }
    }
}

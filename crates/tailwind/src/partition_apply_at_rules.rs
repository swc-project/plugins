use swc_core::{
    common::{util::take::Take, DUMMY_SP},
    css::{
        ast::{
            AtRule, AtRuleName, AtRulePrelude, ComponentValue, DeclarationOrAtRule, Rule,
            SimpleBlock, Stylesheet,
        },
        visit::{VisitMut, VisitMutWith},
    },
};

pub fn partition_apply_at_rules(ss: &mut Stylesheet) {
    ss.visit_mut_with(&mut Visitor::default());
}

#[derive(Debug, Default)]
struct Visitor {
    added: Vec<Rule>,
}

impl VisitMut for Visitor {
    fn visit_mut_rules(&mut self, rules: &mut Vec<Rule>) {
        let mut new = Vec::with_capacity(rules.len());

        for mut rule in rules.take() {
            let prev = self.added.take();
            rule.visit_mut_with(self);

            new.push(rule);
            new.extend(self.added.take());

            self.added = prev;
        }

        *rules = new;
    }

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

                        for nodes in node_groups.into_iter().rev() {
                            let rule = AtRule {
                                span: n.span,
                                name: n.name.clone(),
                                prelude: n.prelude.clone(),
                                block: Some(SimpleBlock {
                                    value: nodes,
                                    ..body.clone()
                                }),
                            };

                            self.added.push(Rule::AtRule(box rule));
                        }
                    }
                    None => {}
                }
            }
        }
    }
}

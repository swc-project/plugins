use swc_atoms::JsWord;
use swc_core::{
    common::collections::AHashSet,
    css::{
        ast::{AtRule, AtRuleName, Stylesheet},
        visit::{Visit, VisitWith},
    },
};

#[derive(Default)]
pub(crate) struct Directives {
    pub tailwindDirectives: AHashSet<JsWord>,

    pub applyDirectives: AHashSet<AtRule>,
}

pub(crate) fn normalize_tailwind_directives(ss: &Stylesheet) -> Directives {
    let mut data = Directives::default();

    let mut v = Visitor { data: &mut data };

    data
}

struct Visitor<'a> {
    data: &'a mut Directives,
}

impl Visit for Visitor<'_> {
    fn visit_at_rule(&mut self, at_rule: &AtRule) {
        if let AtRuleName::Ident(i) = &at_rule.name {
            if &*i.value == "apply" {
                self.data.applyDirectives.insert(at_rule.clone());
            }

            if &*i.value == "tailwind" {
                // self.data.tailwindDirectives.insert(at_rule.params.
                // clone());
            }
        }

        at_rule.visit_children_with(self);
    }
}

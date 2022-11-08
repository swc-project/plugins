use swc_atoms::JsWord;
use swc_core::{
    common::collections::AHashSet,
    css::{
        ast::{AtRule, AtRuleName, AtRulePrelude, ComponentValue, Stylesheet, Token, TokenAndSpan},
        visit::{VisitMut, VisitMutWith},
    },
};

#[derive(Default)]
pub(crate) struct Directives {
    pub tailwind_directives: AHashSet<JsWord>,

    pub apply_directives: AHashSet<AtRule>,
}

pub(crate) fn normalize_tailwind_directives(ss: &mut Stylesheet) -> Directives {
    let mut data = Directives::default();

    let mut v = TailwindNormalizer { data: &mut data };

    ss.visit_mut_with(&mut v);

    data
}

struct TailwindNormalizer<'a> {
    data: &'a mut Directives,
}

impl VisitMut for TailwindNormalizer<'_> {
    fn visit_mut_at_rule(&mut self, at_rule: &mut AtRule) {
        if let AtRuleName::Ident(name) = &at_rule.name {
            if &*name.value == "tailwind" {
                if let Some(box AtRulePrelude::ListOfComponentValues(values)) = &mut at_rule.prelude
                {
                    for v in values.children.iter_mut() {
                        if let ComponentValue::PreservedToken(TokenAndSpan {
                            token: Token::Ident { value, .. },
                            ..
                        }) = v
                        {
                            if &**value == "screens" {
                                *value = "variants".into()
                            }
                        }
                    }
                }
                // self.data.tailwindDirectives.insert(at_rule.params.
                // clone());
            }
        }

        at_rule.visit_mut_children_with(self);
    }
}

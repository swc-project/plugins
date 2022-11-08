use swc_atoms::JsWord;
use swc_core::{
    common::collections::AHashSet,
    css::{
        ast::{
            AtRule, AtRuleName, AtRulePrelude, ComponentValue, ImportPrelude, ImportPreludeHref,
            ListOfComponentValues, Stylesheet, Token, TokenAndSpan,
        },
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
        if let AtRuleName::Ident(name) = &mut at_rule.name {
            if &*name.value == "import" {
                if let Some(box AtRulePrelude::ImportPrelude(ImportPrelude {
                    span, href, ..
                })) = &mut at_rule.prelude
                {
                    let is = |specifier: &str| match &**href {
                        ImportPreludeHref::Url(_) => false,
                        ImportPreludeHref::Str(s) => &s.value == specifier,
                    };

                    let create_prelude = |s: &str| {
                        Some(box AtRulePrelude::ListOfComponentValues(
                            ListOfComponentValues {
                                span: *span,
                                children: vec![ComponentValue::PreservedToken(TokenAndSpan {
                                    span: *span,
                                    token: Token::Ident {
                                        value: s.into(),
                                        raw: "".into(),
                                    },
                                })],
                            },
                        ))
                    };

                    if is("tailwindcss/base") {
                        name.value = "tailwind".into();
                        at_rule.prelude = create_prelude("base");
                    } else if is("tailwindcss/components") {
                        name.value = "tailwind".into();
                        at_rule.prelude = create_prelude("components");
                    } else if is("tailwindcss/components") {
                        name.value = "tailwind".into();
                        at_rule.prelude = create_prelude("utilities");
                    } else if is("tailwindcss/utilities") {
                        name.value = "tailwind".into();
                        at_rule.prelude = create_prelude("components");
                    } else if is("tailwindcss/screens") || is("tailwindcss/variants") {
                        name.value = "tailwind".into();
                        at_rule.prelude = create_prelude("variants");
                    }
                }
            }

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

                            self.data.tailwind_directives.insert(value.clone());
                        }
                    }
                }
            }
        }

        at_rule.visit_mut_children_with(self);
    }
}

use lightningcss::{
    properties::custom::TokenList,
    selector::{Component, Selector},
};
use parcel_selectors::parser::{LocalName, NthOfSelectorData};

pub fn owned_selector<'i>(sel: &Selector) -> Selector<'i> {
    let mut buf: Vec<Component<'i>> = vec![];

    for component in sel.iter_raw_parse_order_from(0) {
        buf.push(owned_component(component));
    }

    Selector::from(buf)
}

pub fn owned_component<'i>(c: &Component) -> Component<'i> {
    match c {
        parcel_selectors::parser::Component::Combinator(v) => {
            parcel_selectors::parser::Component::Combinator(*v)
        }
        parcel_selectors::parser::Component::ExplicitAnyNamespace => {
            parcel_selectors::parser::Component::ExplicitAnyNamespace
        }
        parcel_selectors::parser::Component::ExplicitNoNamespace => {
            parcel_selectors::parser::Component::ExplicitNoNamespace
        }
        parcel_selectors::parser::Component::DefaultNamespace(v) => {
            parcel_selectors::parser::Component::DefaultNamespace(v.clone().into_owned())
        }
        parcel_selectors::parser::Component::Namespace(v1, v2) => {
            parcel_selectors::parser::Component::Namespace(
                v1.clone().into_owned(),
                v2.clone().into_owned(),
            )
        }
        parcel_selectors::parser::Component::ExplicitUniversalType => {
            parcel_selectors::parser::Component::ExplicitUniversalType
        }
        parcel_selectors::parser::Component::ID(v) => {
            parcel_selectors::parser::Component::ID(v.clone().into_owned())
        }
        parcel_selectors::parser::Component::Class(v) => {
            parcel_selectors::parser::Component::Class(v.clone().into_owned())
        }
        parcel_selectors::parser::Component::Root => parcel_selectors::parser::Component::Root,
        parcel_selectors::parser::Component::Empty => parcel_selectors::parser::Component::Empty,
        parcel_selectors::parser::Component::Scope => parcel_selectors::parser::Component::Scope,
        parcel_selectors::parser::Component::PseudoElement(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::Nesting => {
            parcel_selectors::parser::Component::Nesting
        }

        parcel_selectors::parser::Component::AttributeInNoNamespaceExists {
            local_name,
            local_name_lower,
        } => parcel_selectors::parser::Component::AttributeInNoNamespaceExists {
            local_name: local_name.clone().into_owned(),
            local_name_lower: local_name_lower.clone().into_owned(),
        },
        parcel_selectors::parser::Component::AttributeInNoNamespace {
            local_name,
            operator,
            value,
            case_sensitivity,
            never_matches,
        } => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::AttributeOther(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::Negation(v) => {
            parcel_selectors::parser::Component::Negation(owned_selectors(&v))
        }
        parcel_selectors::parser::Component::LocalName(v1) => {
            parcel_selectors::parser::Component::LocalName(LocalName {
                name: v1.name.clone().into_owned(),
                lower_name: v1.lower_name.clone().into_owned(),
            })
        }
        parcel_selectors::parser::Component::Nth(v) => parcel_selectors::parser::Component::Nth(*v),
        parcel_selectors::parser::Component::NthOf(v) => {
            parcel_selectors::parser::Component::NthOf(NthOfSelectorData::new(
                *v.nth_data(),
                owned_selectors(v.selectors()),
            ))
        }
        parcel_selectors::parser::Component::NonTSPseudoClass(v) => {
            parcel_selectors::parser::Component::NonTSPseudoClass(owned_psuedo_class(v))
        }
        parcel_selectors::parser::Component::Slotted(v) => {
            parcel_selectors::parser::Component::Slotted(owned_selector(v))
        }
        parcel_selectors::parser::Component::Part(v) => parcel_selectors::parser::Component::Part(
            v.iter().map(|v| v.clone().into_owned()).collect(),
        ),
        parcel_selectors::parser::Component::Host(v) => {
            parcel_selectors::parser::Component::Host(v.as_ref().map(owned_selector))
        }
        parcel_selectors::parser::Component::Where(v) => {
            parcel_selectors::parser::Component::Where(owned_selectors(v))
        }
        parcel_selectors::parser::Component::Is(v) => {
            parcel_selectors::parser::Component::Is(owned_selectors(v))
        }
        parcel_selectors::parser::Component::Any(v1, v2) => {
            parcel_selectors::parser::Component::Any(*v1, owned_selectors(v2))
        }
        parcel_selectors::parser::Component::Has(v) => {
            parcel_selectors::parser::Component::Has(owned_selectors(v))
        }
    }
}

fn owned_psuedo_class<'i>(
    v: &lightningcss::selector::PseudoClass,
) -> lightningcss::selector::PseudoClass<'i> {
    match v {
        lightningcss::selector::PseudoClass::Lang { languages } => {
            lightningcss::selector::PseudoClass::Lang {
                languages: languages.iter().map(|v| v.clone().into_owned()).collect(),
            }
        }
        lightningcss::selector::PseudoClass::Dir { direction } => {
            lightningcss::selector::PseudoClass::Dir {
                direction: *direction,
            }
        }
        lightningcss::selector::PseudoClass::Hover => lightningcss::selector::PseudoClass::Hover,
        lightningcss::selector::PseudoClass::Active => lightningcss::selector::PseudoClass::Active,
        lightningcss::selector::PseudoClass::Focus => lightningcss::selector::PseudoClass::Focus,
        lightningcss::selector::PseudoClass::FocusVisible => {
            lightningcss::selector::PseudoClass::FocusVisible
        }
        lightningcss::selector::PseudoClass::FocusWithin => {
            lightningcss::selector::PseudoClass::FocusWithin
        }
        lightningcss::selector::PseudoClass::Current => {
            lightningcss::selector::PseudoClass::Current
        }
        lightningcss::selector::PseudoClass::Past => lightningcss::selector::PseudoClass::Past,
        lightningcss::selector::PseudoClass::Future => lightningcss::selector::PseudoClass::Future,
        lightningcss::selector::PseudoClass::Playing => {
            lightningcss::selector::PseudoClass::Playing
        }
        lightningcss::selector::PseudoClass::Paused => lightningcss::selector::PseudoClass::Paused,
        lightningcss::selector::PseudoClass::Seeking => {
            lightningcss::selector::PseudoClass::Seeking
        }
        lightningcss::selector::PseudoClass::Buffering => {
            lightningcss::selector::PseudoClass::Buffering
        }
        lightningcss::selector::PseudoClass::Stalled => {
            lightningcss::selector::PseudoClass::Stalled
        }
        lightningcss::selector::PseudoClass::Muted => lightningcss::selector::PseudoClass::Muted,
        lightningcss::selector::PseudoClass::VolumeLocked => {
            lightningcss::selector::PseudoClass::VolumeLocked
        }
        lightningcss::selector::PseudoClass::Fullscreen(v) => {
            lightningcss::selector::PseudoClass::Fullscreen(*v)
        }
        lightningcss::selector::PseudoClass::Open => lightningcss::selector::PseudoClass::Open,
        lightningcss::selector::PseudoClass::Closed => lightningcss::selector::PseudoClass::Closed,
        lightningcss::selector::PseudoClass::Modal => lightningcss::selector::PseudoClass::Modal,
        lightningcss::selector::PseudoClass::PictureInPicture => {
            lightningcss::selector::PseudoClass::PictureInPicture
        }
        lightningcss::selector::PseudoClass::PopoverOpen => {
            lightningcss::selector::PseudoClass::PopoverOpen
        }
        lightningcss::selector::PseudoClass::Defined => {
            lightningcss::selector::PseudoClass::Defined
        }
        lightningcss::selector::PseudoClass::AnyLink(v) => {
            lightningcss::selector::PseudoClass::AnyLink(*v)
        }
        lightningcss::selector::PseudoClass::Link => lightningcss::selector::PseudoClass::Link,
        lightningcss::selector::PseudoClass::LocalLink => {
            lightningcss::selector::PseudoClass::LocalLink
        }
        lightningcss::selector::PseudoClass::Target => lightningcss::selector::PseudoClass::Target,
        lightningcss::selector::PseudoClass::TargetWithin => {
            lightningcss::selector::PseudoClass::TargetWithin
        }
        lightningcss::selector::PseudoClass::Visited => {
            lightningcss::selector::PseudoClass::Visited
        }
        lightningcss::selector::PseudoClass::Enabled => {
            lightningcss::selector::PseudoClass::Enabled
        }
        lightningcss::selector::PseudoClass::Disabled => {
            lightningcss::selector::PseudoClass::Disabled
        }
        lightningcss::selector::PseudoClass::ReadOnly(v) => {
            lightningcss::selector::PseudoClass::ReadOnly(*v)
        }
        lightningcss::selector::PseudoClass::ReadWrite(v) => {
            lightningcss::selector::PseudoClass::ReadWrite(*v)
        }
        lightningcss::selector::PseudoClass::PlaceholderShown(v) => {
            lightningcss::selector::PseudoClass::PlaceholderShown(*v)
        }
        lightningcss::selector::PseudoClass::Default => {
            lightningcss::selector::PseudoClass::Default
        }
        lightningcss::selector::PseudoClass::Checked => {
            lightningcss::selector::PseudoClass::Checked
        }
        lightningcss::selector::PseudoClass::Indeterminate => {
            lightningcss::selector::PseudoClass::Indeterminate
        }
        lightningcss::selector::PseudoClass::Blank => lightningcss::selector::PseudoClass::Blank,
        lightningcss::selector::PseudoClass::Valid => lightningcss::selector::PseudoClass::Valid,
        lightningcss::selector::PseudoClass::Invalid => {
            lightningcss::selector::PseudoClass::Invalid
        }
        lightningcss::selector::PseudoClass::InRange => {
            lightningcss::selector::PseudoClass::InRange
        }
        lightningcss::selector::PseudoClass::OutOfRange => {
            lightningcss::selector::PseudoClass::OutOfRange
        }
        lightningcss::selector::PseudoClass::Required => {
            lightningcss::selector::PseudoClass::Required
        }
        lightningcss::selector::PseudoClass::Optional => {
            lightningcss::selector::PseudoClass::Optional
        }
        lightningcss::selector::PseudoClass::UserValid => {
            lightningcss::selector::PseudoClass::UserValid
        }
        lightningcss::selector::PseudoClass::UserInvalid => {
            lightningcss::selector::PseudoClass::UserInvalid
        }
        lightningcss::selector::PseudoClass::Autofill(v) => {
            lightningcss::selector::PseudoClass::Autofill(*v)
        }
        lightningcss::selector::PseudoClass::Local { selector } => {
            lightningcss::selector::PseudoClass::Local {
                selector: Box::new(owned_selector(selector)),
            }
        }
        lightningcss::selector::PseudoClass::Global { selector } => {
            lightningcss::selector::PseudoClass::Global {
                selector: Box::new(owned_selector(selector)),
            }
        }
        lightningcss::selector::PseudoClass::WebKitScrollbar(v) => {
            lightningcss::selector::PseudoClass::WebKitScrollbar(*v)
        }
        lightningcss::selector::PseudoClass::Custom { name } => {
            lightningcss::selector::PseudoClass::Custom {
                name: name.clone().into_owned(),
            }
        }
        lightningcss::selector::PseudoClass::CustomFunction { name, arguments } => {
            lightningcss::selector::PseudoClass::CustomFunction {
                name: name.clone().into_owned(),
                arguments: owned_token_list(arguments),
            }
        }
    }
}

fn owned_selectors<'i>(ss: &[Selector]) -> Box<[Selector<'i>]> {
    let mut buf: Vec<Selector<'i>> = vec![];

    for selector in ss.iter() {
        buf.push(owned_selector(selector));
    }

    buf.into_boxed_slice()
}

fn owned_token_list<'i>(t: &TokenList) -> TokenList<'i> {
    TokenList(t.0.iter().map(|v| v.clone().into_owned()).collect())
}

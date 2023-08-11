use lightningcss::{
    properties::custom::TokenList,
    selector::{Component, Selector, ViewTransitionPartName},
};
use parcel_selectors::{
    attr::{AttrSelectorWithOptionalNamespace, NamespaceConstraint, ParsedAttrSelectorOperation},
    parser::{LocalName, NthOfSelectorData},
};

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
            parcel_selectors::parser::Component::PseudoElement(owned_psuedo_element(v))
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
        } => parcel_selectors::parser::Component::AttributeInNoNamespace {
            local_name: local_name.clone().into_owned(),
            operator: *operator,
            value: value.clone().into_owned(),
            case_sensitivity: *case_sensitivity,
            never_matches: *never_matches,
        },
        parcel_selectors::parser::Component::AttributeOther(v) => {
            parcel_selectors::parser::Component::AttributeOther(Box::new(
                AttrSelectorWithOptionalNamespace {
                    namespace: v.namespace.as_ref().map(|v| match v {
                        NamespaceConstraint::Any => NamespaceConstraint::Any,
                        NamespaceConstraint::Specific((v1, v2)) => NamespaceConstraint::Specific((
                            v1.clone().into_owned(),
                            v2.clone().into_owned(),
                        )),
                    }),
                    local_name: v.local_name.clone().into_owned(),
                    local_name_lower: v.local_name_lower.clone().into_owned(),
                    operation: match &v.operation {
                        ParsedAttrSelectorOperation::Exists => ParsedAttrSelectorOperation::Exists,
                        ParsedAttrSelectorOperation::WithValue {
                            operator,
                            case_sensitivity,
                            expected_value,
                        } => ParsedAttrSelectorOperation::WithValue {
                            operator: *operator,
                            case_sensitivity: *case_sensitivity,
                            expected_value: expected_value.clone().into_owned(),
                        },
                    },
                    never_matches: v.never_matches,
                },
            ))
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

fn owned_psuedo_element<'i>(
    v: &lightningcss::selector::PseudoElement,
) -> lightningcss::selector::PseudoElement<'i> {
    match v {
        lightningcss::selector::PseudoElement::After => {
            lightningcss::selector::PseudoElement::After
        }
        lightningcss::selector::PseudoElement::Before => {
            lightningcss::selector::PseudoElement::Before
        }
        lightningcss::selector::PseudoElement::FirstLine => {
            lightningcss::selector::PseudoElement::FirstLine
        }
        lightningcss::selector::PseudoElement::FirstLetter => {
            lightningcss::selector::PseudoElement::FirstLetter
        }
        lightningcss::selector::PseudoElement::Selection(v) => {
            lightningcss::selector::PseudoElement::Selection(v.clone())
        }
        lightningcss::selector::PseudoElement::Placeholder(v) => {
            lightningcss::selector::PseudoElement::Placeholder(v.clone())
        }
        lightningcss::selector::PseudoElement::Marker => {
            lightningcss::selector::PseudoElement::Marker
        }
        lightningcss::selector::PseudoElement::Backdrop(v) => {
            lightningcss::selector::PseudoElement::Backdrop(v.clone())
        }
        lightningcss::selector::PseudoElement::FileSelectorButton(v) => {
            lightningcss::selector::PseudoElement::FileSelectorButton(v.clone())
        }
        lightningcss::selector::PseudoElement::WebKitScrollbar(v) => {
            lightningcss::selector::PseudoElement::WebKitScrollbar(v.clone())
        }
        lightningcss::selector::PseudoElement::Cue => lightningcss::selector::PseudoElement::Cue,
        lightningcss::selector::PseudoElement::CueRegion => {
            lightningcss::selector::PseudoElement::CueRegion
        }
        lightningcss::selector::PseudoElement::CueFunction { selector } => {
            lightningcss::selector::PseudoElement::CueFunction {
                selector: Box::new(owned_selector(selector)),
            }
        }
        lightningcss::selector::PseudoElement::CueRegionFunction { selector } => {
            lightningcss::selector::PseudoElement::CueRegionFunction {
                selector: Box::new(owned_selector(selector)),
            }
        }
        lightningcss::selector::PseudoElement::ViewTransition => {
            lightningcss::selector::PseudoElement::ViewTransition
        }
        lightningcss::selector::PseudoElement::ViewTransitionGroup { part_name } => {
            lightningcss::selector::PseudoElement::ViewTransitionGroup {
                part_name: owned_view_transition_part_name(part_name),
            }
        }
        lightningcss::selector::PseudoElement::ViewTransitionImagePair { part_name } => {
            lightningcss::selector::PseudoElement::ViewTransitionImagePair {
                part_name: owned_view_transition_part_name(part_name),
            }
        }
        lightningcss::selector::PseudoElement::ViewTransitionOld { part_name } => {
            lightningcss::selector::PseudoElement::ViewTransitionOld {
                part_name: owned_view_transition_part_name(part_name),
            }
        }
        lightningcss::selector::PseudoElement::ViewTransitionNew { part_name } => {
            lightningcss::selector::PseudoElement::ViewTransitionNew {
                part_name: owned_view_transition_part_name(part_name),
            }
        }
        lightningcss::selector::PseudoElement::Custom { name } => {
            lightningcss::selector::PseudoElement::Custom {
                name: name.clone().into_owned(),
            }
        }
        lightningcss::selector::PseudoElement::CustomFunction { name, arguments } => {
            lightningcss::selector::PseudoElement::CustomFunction {
                name: name.clone().into_owned(),
                arguments: owned_token_list(arguments),
            }
        }
    }
}

fn owned_view_transition_part_name<'i>(n: &ViewTransitionPartName) -> ViewTransitionPartName<'i> {
    match n {
        ViewTransitionPartName::All => ViewTransitionPartName::All,
        ViewTransitionPartName::Name(v) => ViewTransitionPartName::Name(v.clone().into_owned()),
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
            lightningcss::selector::PseudoClass::WebKitScrollbar(v.clone())
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

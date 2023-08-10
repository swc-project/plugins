use lightningcss::selector::{Component, Selector};
use parcel_selectors::parser::LocalName;

pub fn owned_selector(sel: &Selector) -> Selector<'static> {
    let mut buf: Vec<Component<'static>> = vec![];

    for component in sel.iter_raw_parse_order_from(0) {
        buf.push(owned_component(component));
    }

    Selector::from(buf)
}

pub fn owned_component(c: &Component) -> Component<'static> {
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
            parcel_selectors::parser::Component::Negation(owned_selectors(v))
        }
        parcel_selectors::parser::Component::LocalName(v1) => {
            parcel_selectors::parser::Component::LocalName(LocalName {
                name: v1.name.clone().into_owned(),
                lower_name: v1.lower_name.clone().into_owned(),
            })
        }
        parcel_selectors::parser::Component::Nth(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::NthOf(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::NonTSPseudoClass(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::Slotted(v) => {
            parcel_selectors::parser::Component::Slotted(owned_selector(v))
        }
        parcel_selectors::parser::Component::Part(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::Host(v) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::Where(v) => {
            parcel_selectors::parser::Component::Where(owned_selectors(v))
        }
        parcel_selectors::parser::Component::Is(v) => {
            parcel_selectors::parser::Component::Is(owned_selectors(v))
        }
        parcel_selectors::parser::Component::Any(v1, v2) => {
            unimplemented!()
        }
        parcel_selectors::parser::Component::Has(v) => {
            parcel_selectors::parser::Component::Has(owned_selectors(v))
        }
    }
}

fn owned_selectors(ss: &[Selector]) -> Box<[Selector<'static>]> {
    let mut buf: Vec<Selector<'static>> = vec![];

    for selector in ss.iter() {
        buf.push(owned_selector(selector));
    }

    buf.into_boxed_slice()
}
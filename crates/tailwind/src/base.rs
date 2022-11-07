use swc_core::css::ast::Rule;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Candidate {
    NotOnDemand,
    Str(String),
}
impl Candidate {
    pub(crate) fn starts_with(&self, arg: char) -> bool {
        match self {
            Candidate::NotOnDemand => false,
            Candidate::Str(s) => s.starts_with(arg),
        }
    }

    pub(crate) fn ends_with(&self, arg: char) -> bool {
        match self {
            Candidate::NotOnDemand => false,
            Candidate::Str(s) => s.ends_with(arg),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum LayerNode {
    Base,
    Components,
    Utilities,
    Variant,
}

pub(crate) enum Plugin {
    Function(Box<dyn Fn(&Modifier, &PluginContext) -> Vec<Vec<Rule>>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Modifier {
    Default,
    MinusDefault,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginContext {
    pub is_only_plugin: bool,
}

/// I don't know what is this.
pub(crate) struct RuleOffset {}

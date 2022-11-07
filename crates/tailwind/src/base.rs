use swc_core::css::ast::Rule;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Candidate {
    NotOnDemand,
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
pub(crate) type Sort = ();

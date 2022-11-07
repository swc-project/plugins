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
    Function(Box<dyn Fn()>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Modifier {
    Default,
    MinusDefault,
}

/// I don't know what is this.
pub(crate) type Sort = ();

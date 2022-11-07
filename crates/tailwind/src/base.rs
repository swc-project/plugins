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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct RuleOffset {
    //     * @property {Layer} layer The layer that this rule belongs to
    //  * @property {Layer} parentLayer The layer that this rule originally belonged to. Only
    //    different from layer if this is a variant.
    //  * @property {bigint} arbitrary 0n if false, 1n if true
    //  * @property {bigint} variants Dynamic size. 1 bit per registered variant. 0n means no
    //    variants
    //  * @property {bigint} parallelIndex Rule index for the parallel variant. 0 if not
    //    applicable.
    //  * @property {bigint} index Index of the rule / utility in it's given *parent* layer.
    //    Monotonically increasing.
    // TODO: Declare properties above
    /// Some information on how we can sort arbitrary variants
    pub options: Vec<VariantOption>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct VariantOption {}

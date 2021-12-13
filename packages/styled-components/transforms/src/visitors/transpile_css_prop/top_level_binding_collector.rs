use swc_common::collections::AHashSet;
use swc_ecmascript::{
    ast::{
        ArrowExpr, ClassDecl, FnDecl, Function, ImportDefaultSpecifier, ImportNamedSpecifier,
        ImportStarAsSpecifier, ObjectPatProp, Pat,
    },
    utils::{ident::IdentLike, Id},
    visit::{noop_visit_type, Visit, VisitWith},
};

// Modified from swc_ecma_utils/src/lib.rs:BindingCollector.
pub struct TopLevelBindingCollector {
    bindings: AHashSet<Id>,
}

impl TopLevelBindingCollector {
    fn add(&mut self, i: &Id) {
        self.bindings.insert(i.clone());
    }
}

impl Visit for TopLevelBindingCollector {
    noop_visit_type!();

    fn visit_class_decl(&mut self, node: &ClassDecl) {
        self.add(&node.ident.to_id());
    }

    fn visit_fn_decl(&mut self, node: &FnDecl) {
        self.add(&node.ident.to_id());
    }

    fn visit_pat(&mut self, node: &Pat) {
        match node {
            Pat::Ident(i) => self.add(&i.id.to_id()),
            Pat::Object(o) => {
                for prop in o.props.iter() {
                    match prop {
                        ObjectPatProp::Assign(a) => self.add(&a.key.to_id()),
                        ObjectPatProp::KeyValue(k) => k.value.visit_with(self),
                        ObjectPatProp::Rest(_) => {}
                    }
                }
            }
            Pat::Array(a) => {
                for elem in a.elems.iter() {
                    elem.visit_with(self);
                }
            }
            Pat::Assign(a) => {
                a.left.visit_with(self);
            }
            _ => {}
        }
    }

    fn visit_arrow_expr(&mut self, _: &ArrowExpr) {}
    fn visit_function(&mut self, _: &Function) {}

    fn visit_import_default_specifier(&mut self, node: &ImportDefaultSpecifier) {
        self.add(&node.local.to_id());
    }

    fn visit_import_named_specifier(&mut self, node: &ImportNamedSpecifier) {
        self.add(&node.local.to_id());
    }

    fn visit_import_star_as_specifier(&mut self, node: &ImportStarAsSpecifier) {
        self.add(&node.local.to_id());
    }
}

pub fn collect_top_level_decls<N>(n: &N) -> AHashSet<Id>
where
    N: VisitWith<TopLevelBindingCollector>,
{
    let mut v = TopLevelBindingCollector {
        bindings: Default::default(),
    };
    n.visit_with(&mut v);
    v.bindings
}

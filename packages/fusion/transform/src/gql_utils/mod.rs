use std::cell::RefCell;

use swc_core::{
    common::{collections::AHashMap, SyntaxContext},
    ecma::ast::*,
};

pub use self::analyzer::{analyzer};

mod analyzer;

/// This is created once per file.
#[derive(Debug, Default)]
pub struct State {
    pub(crate) gql_required: Option<Id>,

    unresolved_ctxt: Option<SyntaxContext>,

    imported_local_name: Option<Id>,
    /// Namespace imports
    imported_local_ns: Option<Id>,
    import_name_cache: RefCell<AHashMap<Id, Id>>,
}

impl State {
    pub(crate) fn is_gql(&self, tag: &Expr) -> bool {
        match tag {
            Expr::Ident(_asdf) => {
                if Some(_asdf.to_id()) == self.import_local_name("default", Some(_asdf)) {
                    return true;
                }
            }

            _ => {}
        }
        false
    }

    pub(crate) fn import_local_name(
        &self,
        name: &str,
        cache_identifier: Option<&Ident>,
    ) -> Option<Id> {
        if name == "default" {
            if let Some(cached) = self.imported_local_name.clone() {
                return Some(cached);
            }
            if let Some(cached) = self.imported_local_ns.clone() {
                return Some(cached);
            }
        }

        if let Some(..) = self.imported_local_ns {
            return Some((name.into(), Default::default()));
        }

        let cache_key = cache_identifier.map(|i| i.to_id()).unwrap_or_default();

        let ctxt = self.unresolved_ctxt.unwrap_or_default();

        let local_name = if self.gql_required.is_some() {
            Some(if name == "default" {
                "gql".into()
            } else {
                name.into()
            })
        } else {
            None
        };

        if let Some(cached) = self.import_name_cache.borrow().get(&cache_key) {
            return Some(cached.clone());
        }

        let name = local_name.map(|word| (word, ctxt));

        if let Some(name) = name.clone() {
            self.import_name_cache.borrow_mut().insert(cache_key, name);
        }

        name
    }
}

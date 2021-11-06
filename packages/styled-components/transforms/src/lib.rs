pub use crate::utils::{analyze, analyzer, State};
use crate::visitors::{
    display_name_and_id::display_name_and_id, transpile_css_prop::transpile::transpile_css_prop,
};
use std::{cell::RefCell, rc::Rc};
use swc_common::chain;
use swc_ecmascript::visit::{Fold, VisitMut};

mod css;
mod utils;
mod visitors;

pub fn styled_components() -> impl Fold + VisitMut {
    let state: Rc<RefCell<State>> = Default::default();

    chain!(
        analyzer(state.clone()),
        display_name_and_id(state.clone()),
        transpile_css_prop()
    )
}

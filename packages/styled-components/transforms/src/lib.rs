pub use crate::utils::{analyze, State};
use crate::visitors::{
    display_name_and_id::display_name_and_id, transpile_css_prop::transpile::transpile_css_prop,
};
use std::rc::Rc;
use swc_common::chain;
use swc_ecmascript::{
    ast::Program,
    visit::{Fold, VisitMut},
};

mod css;
mod utils;
mod visitors;

pub fn styled_components(program: &Program) -> impl Fold + VisitMut {
    styled_components_from_state(Rc::new(analyze(program)))
}

pub fn styled_components_from_state(state: Rc<State>) -> impl Fold + VisitMut {
    chain!(display_name_and_id(state.clone()), transpile_css_prop())
}

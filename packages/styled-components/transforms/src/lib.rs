use crate::visitors::{
    display_name_and_id::display_name_and_id, template_literals::transpile::transpile_css_prop,
};
use swc_common::chain;
use swc_ecmascript::visit::{Fold, VisitMut};

mod css;
mod utils;
mod visitors;

pub fn styled_components() -> impl Fold + VisitMut {
    chain!(display_name_and_id(), transpile_css_prop())
}

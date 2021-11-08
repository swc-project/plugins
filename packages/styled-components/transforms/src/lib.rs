pub use crate::utils::{analyze, analyzer, State};
use crate::visitors::{
    display_name_and_id::display_name_and_id, transpile_css_prop::transpile::transpile_css_prop,
};
use serde::Deserialize;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use swc_common::{chain, FileName};
use swc_ecmascript::visit::{Fold, VisitMut};

mod css;
mod utils;
mod visitors;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub display_name: Option<String>,
}

pub fn styled_components(filename: Arc<FileName>, config: Config) -> impl Fold + VisitMut {
    let state: Rc<RefCell<State>> = Default::default();
    let config = Rc::new(config);

    chain!(
        analyzer(state.clone()),
        display_name_and_id(filename.clone(), config.clone(), state.clone()),
        transpile_css_prop()
    )
}

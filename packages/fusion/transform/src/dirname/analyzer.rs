use std::{cell::RefCell, rc::Rc};

use swc_core::ecma::{
    ast::*,
    visit::{as_folder, noop_visit_mut_type, noop_visit_type, Fold, Visit, VisitMut, VisitWith},
};

use super::State;
use crate::Config;

pub fn analyzer(config: Rc<Config>, state: Rc<RefCell<State>>) -> impl VisitMut + Fold {
    as_folder(AsAnalyzer { config, state })
}

struct AsAnalyzer {
    config: Rc<Config>,
    state: Rc<RefCell<State>>,
}

impl VisitMut for AsAnalyzer {
    noop_visit_mut_type!();
}

struct Analyzer<'a> {
    config: &'a Config,
    state: &'a mut State,
}

impl Visit for Analyzer<'_> {
    noop_visit_type!();
}

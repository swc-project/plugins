use super::State;
use crate::Config;
use std::{cell::RefCell, rc::Rc};
use swc_common::DUMMY_SP;
use swc_ecmascript::{
    ast::*,
    utils::ident::IdentLike,
    visit::{
        as_folder, noop_visit_mut_type, noop_visit_type, Fold, Node, Visit, VisitMut, VisitWith,
    },
};

pub fn analyzer(config: Rc<Config>, state: Rc<RefCell<State>>) -> impl VisitMut + Fold {
    as_folder(AsAnalyzer { state })
}

struct AsAnalyzer {
    state: Rc<RefCell<State>>,
}

impl VisitMut for AsAnalyzer {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, p: &mut Module) {
        let mut v = Analyzer {
            state: &mut *self.state.borrow_mut(),
        };

        p.visit_with(&Invalid { span: DUMMY_SP }, &mut v);
    }

    fn visit_mut_script(&mut self, p: &mut Script) {
        let mut v = Analyzer {
            state: &mut *self.state.borrow_mut(),
        };

        p.visit_with(&Invalid { span: DUMMY_SP }, &mut v);
    }
}

pub fn analyze(program: &Program) -> State {
    let mut state = State::default();

    let mut v = Analyzer { state: &mut state };

    program.visit_with(&Invalid { span: DUMMY_SP }, &mut v);

    state
}

struct Analyzer<'a> {
    state: &'a mut State,
}

impl Visit for Analyzer<'_> {
    noop_visit_type!();

    fn visit_import_decl(&mut self, i: &ImportDecl, _: &dyn Node) {
        if &*i.src.value == "styled-components" {
            for s in &i.specifiers {
                match s {
                    ImportSpecifier::Named(_) => {}
                    ImportSpecifier::Default(s) => {
                        self.state.imported_local_name = Some(s.local.to_id());
                    }
                    ImportSpecifier::Namespace(_) => {}
                }
            }
        }
    }
}

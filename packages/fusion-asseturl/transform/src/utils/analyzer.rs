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

    fn visit_mut_module(&mut self, p: &mut Module) {
        let mut v = Analyzer {
            config: &self.config,
            state: &mut self.state.borrow_mut(),
        };

        p.visit_with(&mut v);
    }

    fn visit_mut_script(&mut self, p: &mut Script) {
        let mut v = Analyzer {
            config: &self.config,
            state: &mut self.state.borrow_mut(),
        };

        p.visit_with(&mut v);
    }
}

pub fn analyze(config: &Config, program: &Program) -> State {
    let mut state = State::default();

    let mut v = Analyzer {
        config,
        state: &mut state,
    };

    program.visit_with(&mut v);

    state
}

struct Analyzer<'a> {
    config: &'a Config,
    state: &'a mut State,
}

impl Visit for Analyzer<'_> {
    noop_visit_type!();

    fn visit_import_decl(&mut self, i: &ImportDecl) {
        let is_asseturl = if self.config.top_level_import_paths.is_empty() {
            &*i.src.value == "fusion-core" || i.src.value.starts_with("fusion-core/")
        } else {
            self.config.top_level_import_paths.contains(&i.src.value)
        };

        if is_asseturl {
            for s in &i.specifiers {
                match s {
                    ImportSpecifier::Named(s) => {
                        if true
                            && s.imported
                                .as_ref()
                                .map(|v| match v {
                                    ModuleExportName::Ident(v) => &*v.sym,
                                    ModuleExportName::Str(v) => &*v.value,
                                })
                                .unwrap_or(&*s.local.sym)
                                == "assetUrl"
                        {
                            self.state.imported_local_name = Some(s.local.to_id());
                        }
                    }
                    ImportSpecifier::Default(s) => {
                        self.state.imported_local_name = Some(s.local.to_id());
                    }
                    ImportSpecifier::Namespace(s) => {
                        self.state.imported_local_ns = Some(s.local.to_id());
                    }
                }
            }
        }
    }
}

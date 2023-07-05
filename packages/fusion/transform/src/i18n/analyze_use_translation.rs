use std::{cell::RefCell, rc::Rc};

use swc_core::ecma::{
    ast::*,
    visit::{as_folder, noop_visit_mut_type, noop_visit_type, Fold, Visit, VisitMut, VisitWith},
};
use tracing::debug;

use super::State;

pub fn i18n_analyze_use_translation(state: Rc<RefCell<State>>) -> impl VisitMut + Fold {
    as_folder(AsAnalyzer { state })
}

struct AsAnalyzer {
    state: Rc<RefCell<State>>,
}

impl VisitMut for AsAnalyzer {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, p: &mut Module) {
        let mut v: Analyzer<'_> = Analyzer {
            state: &mut self.state.borrow_mut(),
        };

        p.visit_with(&mut v);
    }

    fn visit_mut_script(&mut self, p: &mut Script) {
        let mut v = Analyzer {
            state: &mut self.state.borrow_mut(),
        };

        p.visit_with(&mut v);
    }
}

struct Analyzer<'a> {
    state: &'a mut State,
}

impl Visit for Analyzer<'_> {
    noop_visit_type!();

    fn visit_var_declarator(&mut self, var_declarator: &VarDeclarator) {
        debug!(
            "use_translation_run: {:?}",
            var_declarator.init.as_ref().unwrap()
        );
        debug!("state: {:?}", self.state);
    }

    fn visit_call_expr(&mut self, call_expr: &CallExpr) {
        match &call_expr.callee {
            Callee::Expr(boxed_expr) => match &**boxed_expr {
                Expr::Ident(ident) => {
                    if self
                        .state
                        .get_use_translation_alias()
                        .contains(ident.sym.as_ref())
                    {
                        match call_expr.args.first() {
                            Some(arg) => match arg {
                                ExprOrSpread { expr, .. } => match &**expr {
                                    &Expr::Lit(ref lit) => match lit {
                                        Lit::Str(lit_str) => {
                                            debug!("---- lit_str: {:?}", lit_str);
                                            self.state.add_translation_id(
                                                lit_str.value.clone().to_string(),
                                            );
                                        }
                                        _ => {}
                                    },
                                    _ => {}
                                },
                            },
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

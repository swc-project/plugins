use swc_ecma_ast::*;
use swc_ecma_visit::{noop_visit_type, visit_pass, Visit, VisitWith};

use super::State;
use crate::Config;

pub fn analyzer<'a>(config: &'a Config, state: &'a mut State) -> impl 'a + Pass {
    visit_pass(Analyzer { config, state })
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
    noop_visit_type!(fail);

    fn visit_var_declarator(&mut self, v: &VarDeclarator) {
        v.visit_children_with(self);

        if let (
            Pat::Ident(name),
            Some(Expr::Call(CallExpr {
                callee: Callee::Expr(callee),
                args,
                ..
            })),
        ) = (&v.name, v.init.as_deref())
        {
            if let Expr::Ident(callee) = &**callee {
                if &*callee.sym == "require" && args.len() == 1 && args[0].spread.is_none() {
                    if let Expr::Lit(Lit::Str(v)) = &*args[0].expr {
                        let is_styled = if self.config.top_level_import_paths.is_empty() {
                            &*v.value == "styled-components"
                                || v.value.starts_with("styled-components/")
                        } else {
                            self.config.top_level_import_paths.contains(&v.value)
                        };

                        if is_styled {
                            self.state.styled_required = Some(name.id.to_id());
                            self.state.unresolved_ctxt = Some(callee.ctxt);
                        }
                    }
                }
            }
        }
    }

    fn visit_import_decl(&mut self, i: &ImportDecl) {
        let is_styled = if self.config.top_level_import_paths.is_empty() {
            &*i.src.value == "styled-components" || i.src.value.starts_with("styled-components/")
        } else {
            self.config.top_level_import_paths.contains(&i.src.value)
        };

        if is_styled {
            for s in &i.specifiers {
                match s {
                    ImportSpecifier::Named(s) => {
                        let imported = s
                            .imported
                            .as_ref()
                            .map(|v| match v {
                                ModuleExportName::Ident(v) => &*v.sym,
                                ModuleExportName::Str(v) => &*v.value,
                            })
                            .unwrap_or(&*s.local.sym);
                        self.state
                            .imported_local_named
                            .insert(imported.to_string(), s.local.to_id());
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

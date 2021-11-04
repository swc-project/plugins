use serde::Deserialize;
use swc_common::{errors::HANDLER, DUMMY_SP};
use swc_ecmascript::{
    ast::*,
    utils::{prepend, private_ident},
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
};

swc_plugin::define_js_plugin!(tester);

fn tester(config: Config) -> impl Fold + VisitMut {
    as_folder(Tester { config })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    #[serde(default)]
    print_error: bool,
    #[serde(default)]
    use_private_ident: bool,
}

struct Tester {
    config: Config,
}

impl VisitMut for Tester {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, m: &mut Module) {
        m.visit_mut_children_with(self);

        if self.config.print_error {
            HANDLER.with(|handler| {
                handler.struct_span_err(m.span, "Test error").emit();
            });
        }

        if self.config.use_private_ident {
            let id = private_ident!("foo");

            prepend(
                &mut m.body,
                ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                    span: DUMMY_SP,
                    expr: Box::new(Expr::Ident(id)),
                })),
            )
        }
    }
}

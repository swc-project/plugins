#![allow(clippy::not_unsafe_ptr_arg_deref)]

use phf::phf_set;
use serde::Deserialize;
use swc_core::plugin::proxies::TransformPluginProgramMetadata;
use swc_ecma_ast::*;
use swc_plugin_macro::plugin_transform;

// swc_plugin::define_js_plugin!(jest);

static HOIST_METHODS: phf::Set<&str> = phf_set![
    "mock",
    "unmock",
    "enableAutomock",
    "disableAutomock",
    "deepUnmock"
];

#[plugin_transform]
fn jest(mut program: Program, _: TransformPluginProgramMetadata) -> Program {
    program.visit_mut_with(&mut Jest);

    program
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {}

struct Jest;

impl Jest {
    fn visit_mut_stmt_like<T>(&mut self, orig: &mut Vec<T>)
    where
        T: StmtLike + VisitMutWith<Self>,
    {
        for item in &mut *orig {
            item.visit_mut_with(self);
        }

        let items = orig.take();

        let mut new = Vec::with_capacity(items.len());
        let mut hoisted = Vec::with_capacity(8);
        items.into_iter().for_each(|item| {
            match item.try_into_stmt() {
                Ok(stmt) => match &stmt {
                    Stmt::Expr(ExprStmt { expr, .. }) => match &**expr {
                        Expr::Call(CallExpr {
                            callee: Callee::Expr(callee),
                            ..
                        }) => match &**callee {
                            Expr::Member(
                                callee @ MemberExpr {
                                    prop: MemberProp::Ident(prop),
                                    ..
                                },
                            ) => match &*callee.obj {
                                Expr::Ident(i) if i.sym == *"jest" => match prop {
                                    _ if HOIST_METHODS.contains(&*prop.sym) => {
                                        hoisted.push(T::from_stmt(stmt));
                                    }
                                    _ => new.push(T::from_stmt(stmt)),
                                },
                                _ => new.push(T::from_stmt(stmt)),
                            },
                            _ => new.push(T::from_stmt(stmt)),
                        },
                        _ => new.push(T::from_stmt(stmt)),
                    },

                    _ => new.push(T::from_stmt(stmt)),
                },
                Err(node) => new.push(node),
            };
        });

        prepend_stmts(&mut new, hoisted.into_iter());

        *orig = new;
    }
}

impl VisitMut for Jest {
    noop_visit_mut_type!();

    fn visit_mut_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        self.visit_mut_stmt_like(stmts)
    }

    fn visit_mut_module_items(&mut self, items: &mut Vec<ModuleItem>) {
        self.visit_mut_stmt_like(items)
    }
}

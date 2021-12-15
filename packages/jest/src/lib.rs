use if_chain::if_chain;
use phf::phf_set;
use serde::Deserialize;
use swc_common::util::take::Take;
use swc_ecmascript::{
    ast::*,
    utils::{prepend_stmts, StmtLike},
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
};

swc_plugin::define_js_plugin!(jest);

static HOIST_METHODS: phf::Set<&str> = phf_set![
    "mock",
    "unmock",
    "enableAutomock",
    "disableAutomock",
    "deepUnmock"
];

fn jest(_: Config) -> impl Fold + VisitMut {
    as_folder(Jest)
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
        for item in items {
            match item.try_into_stmt() {
                Ok(stmt) => if_chain! {
                    if let Stmt::Expr(ExprStmt { expr, .. }) = &stmt;
                    if let Expr::Call(CallExpr { callee: ExprOrSuper::Expr(callee), .. }) = &**expr;
                    if let Expr::Member(callee @ MemberExpr { computed: false, ..  }) = &**callee;
                    if let ExprOrSuper::Expr(callee_obj) = &callee.obj;
                    if let Expr::Ident(i) = &**callee_obj;
                    if i.sym == *"jest";
                    if let Expr::Ident(prop) = &*callee.prop;
                    if HOIST_METHODS.contains(&*prop.sym);
                    then {
                        hoisted.push(T::from_stmt(stmt));
                    } else {
                        new.push(T::from_stmt(stmt));
                    }
                },
                Err(node) => new.push(node),
            }
        }

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

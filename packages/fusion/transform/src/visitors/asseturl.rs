use std::collections::BTreeSet;
use std::{
    cell::RefCell,
    // path::Path,
    rc::Rc,
};

use swc_core::{
    common::{errors::HANDLER, DUMMY_SP},
    ecma::{
        ast::*,
        // atoms::JsWord,
        utils::prepend_stmt,
        visit::{
            as_folder,
            // noop_fold_type,
            noop_visit_mut_type,
            Fold,
            // FoldWith,
            VisitMut,
            VisitMutWith,
        },
    },
};
use tracing::{
    debug,
    span,
    // trace,
    Level,
};

use crate::{asseturl_utils::State, shared::converters::JsVarConverter};

pub fn asseturl(
    // file_name: FileName,
    // src_file_hash: u128,
    // config: Rc<Config>,
    state: Rc<RefCell<State>>,
) -> impl Fold + VisitMut {
    as_folder(DisplayNameAndId {
        state,
        to_prepend: BTreeSet::new(),
        converter: JsVarConverter::new("asseturl"),
    })
}

#[derive(Debug, PartialEq, Hash, Eq, Ord, PartialOrd)]
struct Thing {
    // ident: Ident,
    file_path: String,
}

#[derive(Debug)]
struct DisplayNameAndId {
    state: Rc<RefCell<State>>,
    to_prepend: BTreeSet<Thing>,
    converter: JsVarConverter,
}

impl DisplayNameAndId {
    fn replace_asseturl_call(&mut self, expr: &mut Expr) {
        if let Expr::Call(call_expr) = expr {
            if let Callee::Expr(callee) = &call_expr.callee {
                if let Expr::Ident(_ident) = &**callee {
                    if let Some(ExprOrSpread {
                        expr: expr_other,
                        spread: None,
                    }) = call_expr.args.get(0)
                    {
                        match &**expr_other {
                            Expr::Lit(Lit::Str(lit_str)) => {
                                let src_str = lit_str.value.to_string();
                                let new_ident = self.converter.ident_from_path(&src_str);
                                *expr = new_ident.into();
                                self.to_prepend.insert(Thing { file_path: src_str });
                            }
                            _ => HANDLER.with(|handler| {
                                handler
                                    .err(&format!("asseturl() argument must be a string literal"));
                            }),
                        }
                    }
                }
            }
        }
    }
}

impl VisitMut for DisplayNameAndId {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        n.visit_mut_children_with(self);

        for i in self.to_prepend.iter() {
            let new_ident = self.converter.ident_from_path(&i.file_path);

            let specifier = ImportSpecifier::Default(ImportDefaultSpecifier {
                span: new_ident.span,
                local: new_ident,
            });

            prepend_stmt(
                &mut n.body,
                ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                    span: DUMMY_SP,
                    specifiers: vec![specifier],
                    src: Box::new(Str {
                        span: DUMMY_SP,
                        value: i.file_path.clone().into(),
                        raw: None,
                    }),
                    type_only: Default::default(),
                    asserts: Default::default(),
                })),
            );
        }
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        let is_asseturl = match expr {
            Expr::Call(CallExpr {
                callee: Callee::Expr(callee),
                // args,
                ..
            }) => self.state.borrow().is_asseturl(&*callee),

            _ => false,
        };

        if !is_asseturl {
            return;
        }
        debug!("Found asseturl usage");

        let _tracing = if cfg!(debug_assertions) {
            Some(span!(Level::ERROR, "display_name_and_id").entered())
        } else {
            None
        };

        self.replace_asseturl_call(expr)
    }
}

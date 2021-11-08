use std::{cell::RefCell, rc::Rc};

use crate::utils::{get_prop_name, State};
use swc_atoms::JsWord;
use swc_ecmascript::{
    ast::*,
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
};

pub(crate) fn display_name_and_id(state: Rc<RefCell<State>>) -> impl Fold + VisitMut {
    as_folder(DisplayNameAndId { state })
}

#[derive(Debug)]
struct DisplayNameAndId {
    state: Rc<RefCell<State>>,
}

impl VisitMut for DisplayNameAndId {
    noop_visit_mut_type!();

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        let is_styled = match expr {
            Expr::TaggedTpl(e) => self.state.borrow().is_styled(&e.tag),

            Expr::Call(CallExpr {
                callee: ExprOrSuper::Expr(callee),
                args,
                ..
            }) => {
                (
                    // styled()
                    self.state.borrow().is_styled(&*callee)
                        && get_property_as_ident(&callee)
                            .map(|v| v == "withConfig")
                            .unwrap_or(false)
                ) || (
                    // styled(x)({})
                    self.state.borrow().is_styled(&*callee)
                        && !get_callee(&callee)
                            .map(|callee| callee.is_member())
                            .unwrap_or(false)
                ) || (
                    // styled(x).attrs()({})
                    self.state.borrow().is_styled(callee)
                        && get_callee(&callee)
                            .map(|callee| {
                                callee.is_member()
                                    && get_property_as_ident(&callee)
                                        .map(|v| v == "withConfig")
                                        .unwrap_or(false)
                            })
                            .unwrap_or(false)
                ) || (
                    // styled(x).withConfig({})
                    self.state.borrow().is_styled(&*callee)
                        && get_callee(&callee)
                            .map(|callee| {
                                callee.is_member()
                                    && get_property_as_ident(&callee)
                                        .map(|v| v == "withConfig")
                                        .unwrap_or(false)
                                    && args.len() > 0
                                    && args[0].spread.is_none()
                                    && match &*args[0].expr {
                                        Expr::Object(first_arg) => {
                                            !first_arg.props.iter().any(|prop| match prop {
                                                PropOrSpread::Prop(prop) => {
                                                    match get_prop_name(&prop) {
                                                        Some(PropName::Ident(prop_name)) => {
                                                            match &*prop_name.sym {
                                                                "componentId" | "displayName" => {
                                                                    true
                                                                }
                                                                _ => false,
                                                            }
                                                        }
                                                        _ => false,
                                                    }
                                                }
                                                _ => false,
                                            })
                                        }
                                        _ => false,
                                    }
                            })
                            .unwrap_or(false)
                )
            }

            _ => false,
        };

        if !is_styled {
            return;
        }

        let display_name = if use_display_name(&expr) {
            get_display_name(
                &expr,
                if use_file_name() {
                    Some(&self.state)
                } else {
                    None
                },
            )
        } else {
            None
        };

        add_config(
            e,
            display_name.map(|s| s.replace(DISPLAY_NAME_REGEX, "")),
            if use_ssr(&self.state) {
                Some(get_component_id(&&self.state))
            } else {
                None
            },
        )
    }

    fn visit_mut_member_expr(&mut self, e: &mut MemberExpr) {
        e.obj.visit_mut_with(self);

        if e.computed {
            e.prop.visit_mut_with(self);
        }
    }
}

fn get_callee(e: &Expr) -> Option<&Expr> {
    match e {
        Expr::Call(CallExpr {
            callee: ExprOrSuper::Expr(callee),
            ..
        }) => Some(&callee),
        _ => None,
    }
}

fn get_property_as_ident(e: &Expr) -> Option<&JsWord> {
    match e {
        Expr::Member(MemberExpr {
            prop,
            computed: false,
            ..
        }) => match &**prop {
            Expr::Ident(p) => return Some(&p.sym),
            _ => {}
        },
        _ => {}
    }

    None
}

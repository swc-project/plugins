use swc_ecmascript::{
    ast::*,
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
};

use crate::utils::get_prop_name;

pub(crate) fn display_name_and_id() -> impl Fold + VisitMut {
    as_folder(DisplayNameAndId::default())
}

#[derive(Debug, Default)]
struct DisplayNameAndId {}

impl VisitMut for DisplayNameAndId {
    noop_visit_mut_type!();

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        let is_styled = match expr {
            Expr::TaggedTpl(e) => is_styled(&e.tag),

            Expr::Call(CallExpr {
                callee: ExprOrSuper::Expr(callee),
                args,
                ..
            }) => {
                (
                    // styled()
                    is_styled(&*callee)
                        && get_property_as_ident(&callee)
                            .map(|v| v == "withConfig")
                            .unwrap_or(false)
                ) || (
                    // styled(x)({})
                    is_styled(&*callee)
                        && !get_callee(&callee)
                            .map(|callee| callee.is_member())
                            .unwrap_or(false)
                ) || (
                    // styled(x).attrs()({})
                    is_styled(callee)
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
                    is_styled(&*callee)
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
                                                                _ => [false],
                                                            }
                                                        }
                                                        _ => false,
                                                    }
                                                }
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
    }

    fn visit_mut_member_expr(&mut self, e: &mut MemberExpr) {
        e.obj.visit_mut_with(self);

        if e.computed {
            e.prop.visit_mut_with(self);
        }
    }
}

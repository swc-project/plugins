use crate::{
    utils::{get_name, get_prop_name, prefix_leading_digit, State},
    Config,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{cell::RefCell, path::Path, rc::Rc, sync::Arc};
use swc_atoms::{js_word, JsWord};
use swc_common::{FileName, SourceFile, DUMMY_SP};
use swc_ecmascript::{
    ast::*,
    utils::quote_ident,
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
};

pub(crate) fn display_name_and_id(
    file: Arc<SourceFile>,
    config: Rc<Config>,
    state: Rc<RefCell<State>>,
) -> impl Fold + VisitMut {
    as_folder(DisplayNameAndId {
        file,
        config,
        state,
        component_id: 0,
    })
}

static DISPLAY_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*$").unwrap());

#[derive(Debug)]
struct DisplayNameAndId {
    file: Arc<SourceFile>,
    config: Rc<Config>,
    state: Rc<RefCell<State>>,

    component_id: usize,
}

impl DisplayNameAndId {
    fn get_block_name(&self, p: &Path) -> String {
        let file_stem = p.file_stem();
        if let Some(file_stem) = file_stem {
            if file_stem == "index" {
            } else {
                return file_stem.to_string_lossy().to_string();
            }
        } else {
        }

        self.get_block_name(&p.parent().expect("/index/index/index?"))
    }

    fn get_display_name(&mut self, e: &Expr) -> JsWord {
        let component_name = get_name(e).unwrap_or(js_word!(""));

        match &self.file.name {
            FileName::Real(f) => {
                let block_name = self.get_block_name(f);

                if block_name == &*component_name {
                    return component_name;
                }

                if component_name.is_empty() {
                    return prefix_leading_digit(&block_name).into();
                }

                format!("{}__{}", prefix_leading_digit(&block_name), component_name).into()
            }

            _ => component_name,
        }
    }

    fn next_id(&mut self) -> usize {
        self.component_id += 1;
        self.component_id
    }

    fn get_component_id(&mut self) -> String {
        // Prefix the identifier with a character because CSS classes cannot start with
        // a number

        let next_id = self.next_id();

        format!(
            "{}sc-{}-{}",
            self.config.use_namespace(),
            self.file.src_hash,
            next_id
        )
    }

    fn add_config(&mut self, e: &Expr, display_name: Option<JsWord>, component_id: Option<JsWord>) {
        if display_name.is_none() && component_id.is_none() {
            return;
        }

        let mut with_config_props = vec![];

        if let Some(display_name) = display_name {
            with_config_props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(quote_ident!("displayName")),
                value: Box::new(Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    value: display_name,
                    has_escape: false,
                    kind: Default::default(),
                }))),
            }))))
        }

        if let Some(component_id) = component_id {
            with_config_props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(quote_ident!("componentId")),
                value: Box::new(Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    value: component_id,
                    has_escape: false,
                    kind: Default::default(),
                }))),
            }))))
        }
    }
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

        let display_name = if let Some(display_name) = &self.config.display_name {
            Some(self.get_display_name(&expr))
        } else {
            None
        };

        let component_id = if self.config.use_ssr {
            Some(self.get_component_id().into())
        } else {
            None
        };

        self.add_config(
            expr,
            display_name.map(|s| DISPLAY_NAME_REGEX.replace_all(&*s, "").into()),
            component_id,
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

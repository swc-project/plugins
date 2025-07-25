use std::{convert::TryInto, path::Path};

use once_cell::sync::Lazy;
use regex::Regex;
use swc_atoms::{atom, Atom};
use swc_common::{util::take::Take, DUMMY_SP};
use swc_ecma_ast::*;
use swc_ecma_utils::{quote_ident, ExprFactory};
use swc_ecma_visit::{noop_visit_mut_type, visit_mut_pass, VisitMut, VisitMutWith};
use tracing::{debug, span, trace, Level};

use crate::{
    utils::{get_prop_name, prefix_leading_digit, State},
    Config,
};

pub fn display_name_and_id<'a>(
    file_name: Option<&'a str>,
    src_file_hash: u128,
    config: &'a Config,
    state: &'a State,
) -> impl 'a + Pass {
    visit_mut_pass(DisplayNameAndId {
        file_name,
        src_file_hash,

        config,
        state,
        cur_display_name: Default::default(),
        component_id: 0,
    })
}

static DISPLAY_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9]$").unwrap());

#[derive(Debug)]
struct DisplayNameAndId<'a> {
    file_name: Option<&'a str>,
    src_file_hash: u128,

    config: &'a Config,
    state: &'a State,

    cur_display_name: Option<Atom>,

    component_id: usize,
}

impl DisplayNameAndId<'_> {
    fn get_block_name(&self, p: &Path) -> String {
        match p.file_stem().map(|s| s.to_string_lossy()) {
            Some(file_stem)
                if !self
                    .config
                    .meaningless_file_names
                    .iter()
                    .any(|meaningless| file_stem.as_ref() == meaningless) =>
            {
                file_stem.into()
            }
            _ => self.get_block_name(
                p.parent()
                    .expect("path only contains meaningless filenames (e.g. /index/index)?"),
            ),
        }
    }

    fn get_display_name(&mut self, _: &Expr) -> Atom {
        let component_name = self.cur_display_name.clone().unwrap_or(atom!(""));

        if self.config.file_name {
            if let Some(file_name) = self.file_name {
                let block_name = self.get_block_name(Path::new(file_name));

                if block_name == *component_name {
                    return component_name;
                }

                if component_name.is_empty() {
                    return prefix_leading_digit(&block_name).into();
                }

                return format!("{}__{}", prefix_leading_digit(&block_name), component_name).into();
            }
        }

        component_name
    }

    fn next_id(&mut self) -> usize {
        let ret = self.component_id;
        self.component_id += 1;
        ret
    }

    fn get_component_id(&mut self) -> String {
        // Prefix the identifier with a character because CSS classes cannot start with
        // a number

        let next_id = self.next_id();

        let hash = {
            let base = self.src_file_hash;
            let base = base.to_be_bytes();
            let a = u32::from_be_bytes(base[0..4].try_into().unwrap());
            let b = u32::from_be_bytes(base[4..8].try_into().unwrap());
            let c = u32::from_be_bytes(base[8..12].try_into().unwrap());
            let d = u32::from_be_bytes(base[12..16].try_into().unwrap());

            a ^ b ^ c ^ d
        };

        format!("{}sc-{:x}-{}", self.config.use_namespace(), hash, next_id)
    }

    fn add_config(&mut self, e: &mut Expr, display_name: Option<Atom>, component_id: Option<Atom>) {
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
                    raw: None,
                }))),
            }))))
        }

        if let Some(component_id) = component_id {
            with_config_props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(quote_ident!("componentId")),
                value: Box::new(Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    value: component_id,
                    raw: None,
                }))),
            }))))
        }

        get_existing_config(e, |e| {
            if let Expr::Call(CallExpr { args, .. }) = e {
                if let Some(Expr::Object(existing_config)) = args.get_mut(0).map(|v| &mut *v.expr) {
                    if !already_has(existing_config) {
                        existing_config.props.extend(with_config_props.take());
                    }
                }
            }
        });

        if with_config_props.is_empty() {
            return;
        }

        if let Expr::Call(CallExpr {
            callee: Callee::Expr(callee),
            args,
            ..
        }) = e
        {
            if let Expr::Member(MemberExpr {
                prop: MemberProp::Ident(prop),
                ..
            }) = &**callee
            {
                if &*prop.sym == "withConfig" {
                    if let Some(first_arg) = args.get_mut(0) {
                        if first_arg.spread.is_none() && first_arg.expr.is_object() {
                            if let Expr::Object(obj) = &mut *first_arg.expr {
                                if !already_has(&*obj) {
                                    obj.props.extend(with_config_props);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Expr::TaggedTpl(e) = e {
            e.tag = Box::new(Expr::Call(CallExpr {
                callee: e
                    .tag
                    .take()
                    .make_member(quote_ident!("withConfig"))
                    .as_callee(),
                args: vec![ObjectLit {
                    span: DUMMY_SP,
                    props: with_config_props,
                }
                .as_arg()],
                ..Default::default()
            }));
            return;
        }

        if let Expr::Call(CallExpr {
            callee: Callee::Expr(callee),
            ..
        }) = e
        {
            *callee = Box::new(Expr::Call(CallExpr {
                span: DUMMY_SP,
                callee: callee
                    .take()
                    .make_member(quote_ident!("withConfig"))
                    .as_callee(),
                args: vec![ObjectLit {
                    span: DUMMY_SP,
                    props: with_config_props,
                }
                .as_arg()],
                ..Default::default()
            }));
            return;
        }

        unreachable!("expr should be tagged tpl or call expr");
    }
}

impl VisitMut for DisplayNameAndId<'_> {
    noop_visit_mut_type!(fail);

    fn visit_mut_assign_expr(&mut self, e: &mut AssignExpr) {
        let old = self.cur_display_name.clone();

        if old.is_none() {
            match e.left.as_simple() {
                Some(SimpleAssignTarget::Ident(v)) => {
                    self.cur_display_name = Some(v.sym.clone());
                }
                // Because we visit nodes recursively, this picks the rightmost item in the path,
                // just like babel-plugin-styled-components
                Some(SimpleAssignTarget::Member(MemberExpr { prop, .. })) => {
                    self.cur_display_name = prop.as_ident().map(|v| v.sym.clone());
                }
                _ => {
                    self.cur_display_name = None;
                }
            }
        }

        e.visit_mut_children_with(self);

        self.cur_display_name = old;
    }

    fn visit_mut_class_prop(&mut self, e: &mut ClassProp) {
        let old = self.cur_display_name.take();

        if let PropName::Ident(i) = &e.key {
            self.cur_display_name = Some(i.sym.clone());
        }

        e.visit_mut_children_with(self);

        self.cur_display_name = old;
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        let is_styled = match expr {
            Expr::TaggedTpl(e) => self.state.is_styled(&e.tag),

            Expr::Call(CallExpr {
                callee: Callee::Expr(callee),
                ..
            }) => {
                (
                    // callee is styled.div
                    self.state.is_styled(callee)
                        && get_property_as_ident(callee)
                            .map(|v| v != "withConfig")
                            .unwrap_or(false)
                ) || (
                    // callee is styled(MyComponent)
                    self.state.is_styled(callee)
                        && !get_callee(callee)
                            .map(|callee| callee.is_member())
                            .unwrap_or(false)
                ) || (
                    // callee is styled(MyComponent).attrs(...)
                    self.state.is_styled(callee)
                        && get_callee(callee)
                            .and_then(get_property_as_ident)
                            .map(|v| v != "withConfig")
                            .unwrap_or(false)
                ) || (
                    // callee is styled(MyComponent).withConfig({ ... }), and componentId or
                    // displayName is not set
                    self.state.is_styled(callee)
                        && get_callee(callee)
                            .and_then(get_property_as_ident)
                            .map(|v| v == "withConfig")
                            .unwrap_or(false)
                        && Expr::as_call(callee)
                            .and_then(|with_config_call| with_config_call.args.first())
                            .filter(|first_arg| first_arg.spread.is_none())
                            .and_then(|first_arg| first_arg.expr.as_object())
                            .map(|first_arg_obj| !already_has(first_arg_obj))
                            .unwrap_or(false)
                )
            }

            _ => false,
        };

        if !is_styled {
            return;
        }
        debug!("Found styled component");

        let _tracing = if cfg!(debug_assertions) {
            Some(span!(Level::ERROR, "display_name_and_id").entered())
        } else {
            None
        };

        let display_name = self
            .config
            .display_name
            .then(|| self.get_display_name(expr));
        trace!("display_name: {:?}", display_name);

        let component_id = self.config.ssr.then(|| self.get_component_id().into());
        trace!("component_id: {:?}", display_name);

        self.add_config(
            expr,
            display_name.map(|s| DISPLAY_NAME_REGEX.replace_all(&s, "").into()),
            component_id,
        )
    }

    fn visit_mut_key_value_prop(&mut self, e: &mut KeyValueProp) {
        let old = self.cur_display_name.take();

        if let PropName::Ident(name) = &e.key {
            self.cur_display_name = Some(name.sym.clone());
        }

        e.visit_mut_children_with(self);

        self.cur_display_name = old;
    }

    fn visit_mut_var_declarator(&mut self, v: &mut VarDeclarator) {
        let old = self.cur_display_name.take();

        if let Pat::Ident(name) = &v.name {
            self.cur_display_name = Some(name.id.sym.clone());
        }

        v.visit_mut_children_with(self);

        self.cur_display_name = old;
    }
}

fn get_callee(e: &Expr) -> Option<&Expr> {
    match e {
        Expr::Call(CallExpr {
            callee: Callee::Expr(callee),
            ..
        }) => Some(callee),
        _ => None,
    }
}

fn get_property_as_ident(e: &Expr) -> Option<&Atom> {
    if let Expr::Member(MemberExpr {
        prop: MemberProp::Ident(p),
        ..
    }) = e
    {
        return Some(&p.sym);
    }

    None
}

fn already_has(obj: &ObjectLit) -> bool {
    obj.props
        .iter()
        .filter_map(|v| match v {
            PropOrSpread::Prop(p) => Some(p),
            _ => None,
        })
        .filter_map(|v| get_prop_name(v))
        .any(|prop| match prop {
            PropName::Ident(ident) => &*ident.sym == "componentId" || &*ident.sym == "displayName",
            _ => false,
        })
}

fn get_existing_config<F>(e: &mut Expr, op: F)
where
    F: FnOnce(&mut Expr),
{
    if let Expr::Call(CallExpr {
        callee: Callee::Expr(callee),
        ..
    }) = e
    {
        if let Expr::Call(CallExpr {
            callee: Callee::Expr(callee_callee),
            ..
        }) = &mut **callee
        {
            if let Expr::Member(MemberExpr {
                prop: MemberProp::Ident(prop),
                ..
            }) = &**callee_callee
            {
                if &*prop.sym == "withConfig" {
                    return op(callee);
                }
            }

            if let Expr::Member(MemberExpr {
                obj,
                prop: MemberProp::Ident(..),
                ..
            }) = &mut **callee_callee
            {
                if let Expr::Call(CallExpr {
                    callee: Callee::Expr(callee),
                    ..
                }) = &**obj
                {
                    if let Expr::Member(MemberExpr {
                        prop: MemberProp::Ident(prop),
                        ..
                    }) = &**callee
                    {
                        if &*prop.sym == "withConfig" {
                            op(obj)
                        }
                    }
                }
            }
        }
    }
}

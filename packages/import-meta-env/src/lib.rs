#![allow(clippy::not_unsafe_ptr_arg_deref)]

use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{Expr, Ident, MemberExpr, MetaPropKind, Program},
        visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

pub struct ImportMetaEnv;

impl VisitMut for ImportMetaEnv {
    noop_visit_mut_type!();

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        if let Expr::Member(member) = expr {
            if is_import_meta_env(member) {
                member.obj = Box::new(Ident::new_no_ctxt("process".into(), DUMMY_SP).into());
            }
        }
    }
}

fn is_import_meta_env(member: &MemberExpr) -> bool {
    let Some(obj) = member.obj.as_meta_prop() else {
        return false;
    };

    let Some(prop) = member.prop.as_ident() else {
        return false;
    };

    obj.kind == MetaPropKind::ImportMeta && prop.sym == "env"
}

#[plugin_transform]
fn swc_plugin_import_meta_env(
    mut program: Program,
    _metadata: TransformPluginProgramMetadata,
) -> Program {
    program.visit_mut_with(&mut ImportMetaEnv);
    program
}

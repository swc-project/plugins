#![allow(clippy::not_unsafe_ptr_arg_deref)]

use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{Expr, Ident, MemberExpr, MetaPropKind, Program},
        visit::{noop_visit_mut_type, visit_mut_pass, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    noop_visit_mut_type!();

    fn visit_mut_expr(&mut self, n: &mut Expr) {
        n.visit_mut_children_with(self);

        if let Expr::Member(member) = n {
            if is_import_meta_env(member) {
                member.obj = Box::new(Ident::new_no_ctxt("process".into(), DUMMY_SP).into());
            }
        }
    }
}

fn is_import_meta_env(n: &MemberExpr) -> bool {
    let Some(obj) = n.obj.as_meta_prop() else {
        return false;
    };
    let Some(prop) = n.prop.as_ident() else {
        return false;
    };

    obj.kind == MetaPropKind::ImportMeta && prop.sym == "env"
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.apply(&mut visit_mut_pass(TransformVisitor))
}

#[cfg(test)]
mod tests {
    use swc_core::ecma::{
        parser::{EsSyntax, Syntax},
        transforms::testing::test,
        visit::visit_mut_pass,
    };

    use super::*;

    fn syntax() -> Syntax {
        Syntax::Es(EsSyntax {
            import_attributes: true,
            ..Default::default()
        })
    }

    test!(
        syntax(),
        |_| visit_mut_pass(TransformVisitor),
        transform_import_meta_env,
        r#"import.meta.env"#
    );

    test!(
        syntax(),
        |_| visit_mut_pass(TransformVisitor),
        transform_import_meta_env_prop,
        r#"import.meta.env.MODE"#
    );

    test!(
        syntax(),
        |_| visit_mut_pass(TransformVisitor),
        transform_import_meta_env_key,
        r#"import.meta.env["PROP"]"#
    );

    test!(
        syntax(),
        |_| visit_mut_pass(TransformVisitor),
        no_transform_import_meta,
        r#"import.meta"#
    );

    test!(
        syntax(),
        |_| visit_mut_pass(TransformVisitor),
        no_transform_import_meta_glob,
        r#"import.meta.glob"#
    );
}

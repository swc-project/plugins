#![allow(clippy::not_unsafe_ptr_arg_deref)]

use swc_common::DUMMY_SP;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_ecma_ast::{Expr, Ident, MemberExpr, MetaPropKind, Program};
use swc_ecma_visit::{noop_visit_mut_type, visit_mut_pass, VisitMut, VisitMutWith};

// Adapted from https://github.com/Codex-/swc-plugin-import-meta-env (Apache-2.0).
struct ImportMetaEnv;

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
fn swc_plugin(program: Program, _: TransformPluginProgramMetadata) -> Program {
    program.apply(visit_mut_pass(ImportMetaEnv))
}

#[cfg(test)]
mod tests {
    use swc_ecma_transforms_testing::test_inline;

    use super::*;

    test_inline!(
        Default::default(),
        |_| visit_mut_pass(ImportMetaEnv),
        env_object,
        "import.meta.env;",
        "process.env;"
    );

    test_inline!(
        Default::default(),
        |_| visit_mut_pass(ImportMetaEnv),
        env_properties,
        r#"[import.meta.env.MODE, import.meta.env["PROP"], import.meta.env?.MODE];"#,
        r#"[process.env.MODE, process.env["PROP"], process.env?.MODE];"#
    );

    test_inline!(
        Default::default(),
        |_| visit_mut_pass(ImportMetaEnv),
        nested_expressions,
        "function read() { return useEnv(import.meta.env, () => import.meta.env.MODE); }",
        "function read() { return useEnv(process.env, () => process.env.MODE); }"
    );

    test_inline!(
        Default::default(),
        |_| visit_mut_pass(ImportMetaEnv),
        assignment,
        "import.meta.env.MODE = import.meta.env.OTHER;",
        "process.env.MODE = process.env.OTHER;"
    );

    test_inline!(
        Default::default(),
        |_| visit_mut_pass(ImportMetaEnv),
        unrelated_expressions,
        r#"[import.meta, import.meta.url, import.meta.glob("./*.js"), meta.env, process.env, import.meta.environment, import.meta["env"]];"#,
        r#"[import.meta, import.meta.url, import.meta.glob("./*.js"), meta.env, process.env, import.meta.environment, import.meta["env"]];"#
    );
}

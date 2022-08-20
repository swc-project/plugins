use swc_core::{
    ast::*,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
    visit::{VisitMut, VisitMutWith},
};

#[plugin_transform]
fn loadable_components_plugin(
    mut program: Program,
    data: TransformPluginProgramMetadata,
) -> Program {
    program.visit_mut_with(&mut loadable_transform());

    program
}

pub fn loadable_transform() -> impl VisitMut {
    Loadable {}
}

struct Loadable {}

impl Loadable {
    fn is_valid_identifier(e: &Expr) -> bool {
        match e {
            Expr::Ident(i) => &*i.sym == "loadable",
            Expr::Call(CallExpr {
                callee: Callee::Expr(callee),
                ..
            }) => match &**callee {
                Expr::Member(MemberExpr {
                    obj,
                    prop: MemberProp::Ident(prop),
                    ..
                }) => match &**obj {
                    Expr::Ident(i) => &*i.sym == "loadable" && &*prop.sym == "lib",
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }
}

impl VisitMut for Loadable {}

//! Port of https://github.com/styled-components/babel-plugin-styled-components/blob/a20c3033508677695953e7a434de4746168eeb4e/src/visitors/transpileCssProp.js

use swc_common::DUMMY_SP;
use swc_ecmascript::{
    ast::*,
    utils::{prepend, private_ident},
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
};

pub fn transpile_css_prop() -> impl Fold + VisitMut {
    as_folder(TranspileCssProp::default())
}

#[derive(Default)]
struct TranspileCssProp {
    import_name: Option<Ident>,
}

impl VisitMut for TranspileCssProp {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, n: &mut Module) {
        // TODO: Skip if there are no css prop usage
        n.visit_mut_children_with(self);

        if let Some(import_name) = self.import_name.take() {
            let specifier = ImportSpecifier::Default(ImportDefaultSpecifier {
                span: DUMMY_SP,
                local: import_name,
            });
            prepend(
                &mut n.body,
                ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                    span: DUMMY_SP,
                    specifiers: vec![specifier],
                    src: Str {
                        span: DUMMY_SP,
                        value: "styled-components".into(),
                        has_escape: Default::default(),
                        kind: Default::default(),
                    },
                    type_only: Default::default(),
                    asserts: Default::default(),
                })),
            );
        }
    }

    fn visit_mut_jsx_attr(&mut self, n: &mut JSXAttr) {
        n.visit_mut_children_with(self);

        if !matches!(&n.name,JSXAttrName::Ident(i) if &*i.sym == "css") {
            return;
        }

        let import_name = self
            .import_name
            .get_or_insert_with(|| private_ident!("styled"))
            .clone();

        n.visit_mut_with(&mut transpile_css_prop());
    }
}

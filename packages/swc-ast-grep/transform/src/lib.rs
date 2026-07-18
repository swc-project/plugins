#![feature(box_patterns)]
#![feature(never_type)]

use swc_common::{comments::Comments, util::take::Take, Mark};
use swc_ecma_ast::{Module, ModuleDecl, ModuleItem, VarDeclarator};
use swc_ecma_visit::{VisitMut, VisitMutWith};

use crate::{config::Config, import_analyzer::ImportMap};

pub mod config;
mod flag;
mod import_analyzer;

#[derive(Debug, Clone)]
pub struct Env {
    pub unresolved_mark: Mark,
}

pub fn swc_sdk<C>(env: Env, config: Config, comments: C) -> impl VisitMut
where
    C: Comments,
{
    SwcSdkTransform {
        env,
        config,
        comments,
        imports: Default::default(),
    }
}

/// Handles functions from `@swc/sdk`.
struct SwcSdkTransform<C>
where
    C: Comments,
{
    #[allow(unused)]
    env: Env,
    config: Config,
    #[allow(unused)]
    comments: C,
    imports: ImportMap,
}

impl<C> VisitMut for SwcSdkTransform<C>
where
    C: Comments,
{
    fn visit_mut_var_declarator(&mut self, node: &mut VarDeclarator) {
        self.transform_flag(node);

        node.visit_mut_children_with(self);
    }

    fn visit_mut_module(&mut self, m: &mut Module) {
        self.imports = ImportMap::analyze(m);

        m.visit_mut_children_with(self);
    }

    fn visit_mut_module_item(&mut self, m: &mut ModuleItem) {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = m {
            if self.config.remove_imports_from.contains(&import.src.value) {
                m.take();
                return;
            }
        }

        m.visit_mut_children_with(self);
    }
}

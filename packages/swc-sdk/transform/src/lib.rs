#![feature(box_patterns)]
#![feature(never_type)]

use rustc_hash::FxHashMap;
use swc_common::{comments::Comments, util::take::Take, Mark};
use swc_ecma_ast::{Id, Module, ModuleDecl, ModuleItem, Str, VarDeclarator};
use swc_ecma_utils::find_pat_ids;
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
        ids_to_make_dynamic: Default::default(),
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

    ids_to_make_dynamic: FxHashMap<Id, Str>,
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

        for item in &m.body {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item {
                if self.comments.has_flag(import.span.lo, "DYNAMIC") {
                    let ids: Vec<Id> = find_pat_ids(&import.specifiers);

                    for id in ids {
                        self.ids_to_make_dynamic.insert(id, *import.src.clone());
                    }
                }
            }
        }

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

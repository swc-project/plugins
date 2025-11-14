#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![feature(box_patterns)]

use swc_core::plugin::proxies::TransformPluginProgramMetadata;
use swc_ecma_ast::{Module, Program};
use swc_ecma_visit::{VisitMut, VisitMutWith};
use swc_plugin_macro::plugin_transform;

use crate::import_map::ImportMap;

mod import_map;

#[plugin_transform]
fn next_intl_plugin(mut program: Program, _: TransformPluginProgramMetadata) -> Program {
    program.visit_mut_with(&mut TransformVisitor {
        imports: ImportMap::default(),
    });

    program
}

struct TransformVisitor {
    imports: ImportMap,
}

impl VisitMut for TransformVisitor {
    fn visit_mut_module(&mut self, module: &mut Module) {
        self.imports = ImportMap::analyze(module);

        module.visit_mut_children_with(self);
    }
}

#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![feature(box_patterns)]

use swc_core::plugin::proxies::TransformPluginProgramMetadata;
use swc_ecma_ast::{
    Id, ImportSpecifier, Module, ModuleDecl, ModuleExportName, ModuleItem, Program,
};
use swc_ecma_utils::private_ident;
use swc_ecma_visit::{VisitMut, VisitMutWith};
use swc_plugin_macro::plugin_transform;

use crate::import_map::ImportMap;

mod import_map;

#[plugin_transform]
fn next_intl_plugin(mut program: Program, _: TransformPluginProgramMetadata) -> Program {
    program.visit_mut_with(&mut TransformVisitor {
        imports: Default::default(),
        hook_type: Default::default(),
        hook_local_name: Default::default(),
    });

    program
}

struct TransformVisitor {
    imports: ImportMap,

    hook_type: Option<HookType>,
    hook_local_name: Option<Id>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum HookType {
    UseTranslation,
    GetTranslation,
}

impl VisitMut for TransformVisitor {
    fn visit_mut_module(&mut self, module: &mut Module) {
        self.imports = ImportMap::analyze(module);

        for import in module.body.iter_mut() {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = import {
                match import.src.value.as_bytes() {
                    b"next-intl" => {
                        for specifier in &mut import.specifiers {
                            if let ImportSpecifier::Named(named_spec) = specifier {
                                let orig_name = named_spec
                                    .imported
                                    .as_ref()
                                    .and_then(|x| match x {
                                        ModuleExportName::Ident(ident) => Some(ident.sym.clone()),
                                        ModuleExportName::Str(str) => None,
                                    })
                                    .unwrap_or_else(|| named_spec.local.sym.clone())
                                    .clone();

                                if orig_name == "getExtracted" {
                                    self.hook_type = Some(HookType::UseTranslation);
                                    self.hook_local_name = Some(named_spec.local.to_id());

                                    named_spec.imported = None;
                                    named_spec.local = private_ident!("useTranslation");
                                    break;
                                }
                            }
                        }
                    }

                    b"next-intl/server" => {
                        for specifier in &mut import.specifiers {
                            if let ImportSpecifier::Named(named_spec) = specifier {
                                let orig_name = named_spec
                                    .imported
                                    .as_ref()
                                    .and_then(|x| match x {
                                        ModuleExportName::Ident(ident) => Some(ident.sym.clone()),
                                        ModuleExportName::Str(str) => None,
                                    })
                                    .unwrap_or_else(|| named_spec.local.sym.clone())
                                    .clone();

                                if orig_name == "getExtracted" {
                                    self.hook_type = Some(HookType::GetTranslation);
                                    self.hook_local_name = Some(named_spec.local.to_id());

                                    named_spec.imported = None;
                                    named_spec.local = private_ident!("getTranslations");
                                    break;
                                }
                            }
                        }
                    }

                    _ => {}
                }
            }
        }

        module.visit_mut_children_with(self);
    }
}

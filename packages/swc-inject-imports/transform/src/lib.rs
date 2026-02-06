#![feature(box_patterns)]

use serde::Deserialize;
use swc_atoms::Atom;
use swc_common::util::take::Take;
use swc_ecma_ast::{ImportDecl, Module, ModuleDecl, ModuleItem, Pass, Program, Str};
use swc_ecma_utils::prepend_stmts;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub imports_paths: Vec<Atom>,

    #[serde(default)]
    pub only_filenames: Vec<Atom>,
}

pub fn swc_inject_imports(filename: Atom, config: Config) -> impl Pass {
    InjectImports { config, filename }
}

struct InjectImports {
    config: Config,
    filename: Atom,
}

impl Pass for InjectImports {
    fn process(&mut self, program: &mut swc_ecma_ast::Program) {
        if self.config.imports_paths.is_empty() {
            return;
        }

        if !self.config.only_filenames.is_empty() {
            let mut find = false;
            for filename_filter in self.config.only_filenames.iter() {
                if self.filename.ends_with(&**filename_filter) {
                    find = true;
                    break;
                }
            }
            if !find {
                return;
            }
        }

        let Program::Module(Module { body, .. }) = program else {
            return;
        };

        let new_imports = self.config.imports_paths.iter().map(|path| {
            ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                src: Box::new(Str::from(path.clone())),
                specifiers: vec![],
                ..Take::dummy()
            }))
        });

        prepend_stmts(body, new_imports);
    }
}

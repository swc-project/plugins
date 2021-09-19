use swc_ecma_ast::*;
use swc_ecma_utils::{ident::IdentLike, Id};
use swc_ecma_visit::{as_folder, noop_visit_mut_type, Fold, VisitMut};

pub fn emotion_plugin(config: Config) -> impl VisitMut + Fold {
    as_folder(Emotion {
        config,
        state: Default::default(),
    })
}

#[derive(Debug)]
pub struct Config {}

#[derive(Debug, Default)]
struct State {
    /// Imports from `@emotion/css`.
    ///
    /// e.g. `import { css } from "@emotion/css";`
    vanilla_css: Vec<Id>,
}

struct Emotion {
    #[allow(dead_code)]
    config: Config,
    state: State,
}

impl VisitMut for Emotion {
    // Reduce binary size.
    noop_visit_mut_type!();

    fn visit_mut_import_decl(&mut self, i: &mut ImportDecl) {
        match &*i.src.value {
            "@emotion/css" => {
                self.state
                    .vanilla_css
                    .extend(find_import(&i.specifiers, "css"));
            }
            _ => {}
        }
    }
}

fn find_import(ss: &[ImportSpecifier], wanted: &str) -> Vec<Id> {
    let mut ids = vec![];

    for s in ss {
        match s {
            ImportSpecifier::Named(n) => match &n.imported {
                Some(imported) => {
                    if imported.sym == *wanted {
                        ids.push(n.local.to_id());
                    }
                }
                None => {
                    if n.local.sym == *wanted {
                        ids.push(n.local.to_id());
                    }
                }
            },
            _ => {
                todo!("default import and namespacaed import for emotion plugins")
            }
        }
    }

    ids
}

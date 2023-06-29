use std::{cell::RefCell, rc::Rc};

use swc_core::{
    common::{errors::HANDLER, FileName},
    ecma::{
        ast::*,
        visit::{
            as_folder, noop_visit_mut_type, noop_visit_type, Fold, Visit, VisitMut, VisitWith,
        },
    },
};
use tracing::{debug};

use super::State;

pub fn analyzer(file_name: FileName, state: Rc<RefCell<State>>) -> impl VisitMut + Fold {
    as_folder(AsAnalyzer { file_name, state })
}

struct AsAnalyzer {
    file_name: FileName,
    state: Rc<RefCell<State>>,
}

impl VisitMut for AsAnalyzer {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, p: &mut Module) {
        let mut v: Analyzer<'_> = Analyzer {
            file_name: &self.file_name,
            state: &mut self.state.borrow_mut(),
        };

        p.visit_with(&mut v);
    }

    fn visit_mut_script(&mut self, p: &mut Script) {
        let mut v = Analyzer {
            file_name: &self.file_name,
            state: &mut self.state.borrow_mut(),
        };

        p.visit_with(&mut v);
    }
}

pub fn find_id_attribute(opening_element: &JSXOpeningElement) -> Option<String> {
    for attr_or_spread in &opening_element.attrs {
        match attr_or_spread {
            JSXAttrOrSpread::JSXAttr(attr) => {
                match &attr.name {
                    JSXAttrName::Ident(ident) => {
                        // `sym` is a `string_cache::Atom` and needs to be converted to a string for
                        // comparison
                        if ident.sym.as_ref() == "id" {
                            match &attr.value {
                                Some(JSXAttrValue::Lit(lit)) => {
                                    match &lit {
                                        Lit::Str(lit_str) => {
                                            // `LitStr` has a `value` field that is a `JsWord` type
                                            // which also needs to be converted to a string for use
                                            return Some(lit_str.value.as_ref().to_string());
                                        }
                                        _ => HANDLER.with(|handler| {
                                            handler.err(&format!(
                                                "The translate component must have props.id being \
                                                 a string literal."
                                            ));
                                        }),
                                    }
                                }
                                _ => HANDLER.with(|handler| {
                                    handler.err(&format!(
                                        "The translate component must have props.id being a \
                                         string literal."
                                    ));
                                }),
                            }
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
    None
}

struct Analyzer<'a> {
    file_name: &'a FileName,
    state: &'a mut State,
}

impl Visit for Analyzer<'_> {
    noop_visit_type!();

    

    fn visit_jsx_opening_element(&mut self, opening_element: &JSXOpeningElement) {
        debug!("visit_jsx_opening_element: {:?}", opening_element);
        debug!("imports to test: {:?}", self.state.get_fusion_plugin_imports());
        match &opening_element.name {
            JSXElementName::Ident(ident) => {
                if self.state.get_fusion_plugin_imports().contains(ident.sym.as_ref()) {
                    let attr_id_value = find_id_attribute(opening_element);
                    match attr_id_value {
                        Some(id) => {
                            self.state.add_translation_id(id);
                        }
                        None => (),
                    }
                }
            }
            _ => (),
        }
    }

    fn visit_import_decl(&mut self, i: &ImportDecl) {
        debug!("file_name: {:?}", self.file_name);
        if &*i.src.value == "fusion-plugin-i18n-react" {
            for s in &i.specifiers {
                match s {
                    ImportSpecifier::Named(s) => {
                        if true
                            && s.imported
                                .as_ref()
                                .map(|v| match v {
                                    ModuleExportName::Ident(v) => &*v.sym,
                                    ModuleExportName::Str(v) => &*v.value,
                                })
                                .unwrap_or(&*s.local.sym)
                                == "Translate"
                        {
                            self.state.add_fusion_plugin_import(s.local.sym.as_ref().to_owned());
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}

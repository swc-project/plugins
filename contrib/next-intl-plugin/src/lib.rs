#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![feature(box_patterns)]

use rustc_hash::FxHashMap;
use swc_atoms::Wtf8Atom;
use swc_core::plugin::proxies::TransformPluginProgramMetadata;
use swc_ecma_ast::*;
use swc_ecma_utils::private_ident;
use swc_ecma_visit::{VisitMut, VisitMutWith};
use swc_plugin_macro::plugin_transform;

#[plugin_transform]
fn next_intl_plugin(mut program: Program, _: TransformPluginProgramMetadata) -> Program {
    program.visit_mut_with(&mut TransformVisitor {
        hook_type: Default::default(),
        hook_local_name: Default::default(),
        translator_map: Default::default(),
    });

    program
}

struct TransformVisitor {
    hook_type: Option<HookType>,
    hook_local_name: Option<Id>,

    translator_map: FxHashMap<Id, TranslatorInfo>,
}

#[derive(Debug, Clone)]
struct TranslatorInfo {
    namespace: Option<Wtf8Atom>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum HookType {
    UseTranslation,
    GetTranslation,
}

impl VisitMut for TransformVisitor {
    fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
        let mut is_translator_call = false;
        let mut namespace = None;

        // Handle Identifier case: t("message")
        match &call.callee {
            Callee::Expr(box Expr::Ident(ident)) => {
                if let Some(translator) = self.translator_map.get(&ident.to_id()) {
                    is_translator_call = true;
                    namespace = translator.namespace.clone();
                }
            }

            Callee::Expr(box Expr::Member(MemberExpr {
                span,
                obj: box Expr::Ident(obj),
                prop: MemberProp::Ident(prop),
            })) => {
                if matches!(&*prop.sym, "rich" | "markup" | "has") {
                    if let Some(translator) = self.translator_map.get(&obj.to_id()) {
                        is_translator_call = true;
                        namespace = translator.namespace.clone();
                    }
                }
            }

            _ => {}
        }

        if is_translator_call {
            let arg0 = call.args.first();

            let mut message_text = None;
            let mut explicit_id = None;
            let mut description = None;
            let mut values_node = None;
            let mut formats_node = None;

            if let Some(arg0) = arg0 {
                match &*arg0.expr {
                    // Handle object syntax: t({id: 'key', message: 'text'})
                    Expr::Object(ObjectLit { props, .. }) => {
                        for prop in props {
                            if let PropOrSpread::Prop(box Prop::KeyValue(KeyValue {
                                key,
                                value,
                                ..
                            })) = prop
                            {
                                if let PropName::Ident(key) = key {
                                    let static_id = extract_static_string(value);
                                    if let Some(static_id) = static_id {
                                        explicit_id = Some(static_id);
                                    }
                                } else if key.sym == "message" {
                                    let static_message = extract_static_string(value);
                                    if let Some(static_message) = static_message {
                                        message_text = Some(static_message);
                                    } else {
                                        warn_dynamic_expression(value);
                                    }
                                } else if key.sym == "description" {
                                    let static_description = extract_static_string(value);
                                    if let Some(static_description) = static_description {
                                        description = Some(static_description);
                                    } else {
                                        warn_dynamic_expression(value);
                                    }
                                } else if key.sym == "values" {
                                    values_node = Some(value);
                                } else if key.sym == "formats" {
                                    formats_node = Some(value);
                                }
                            }
                        }
                    }

                    // Handle string syntax: t('text') or t(`text`)
                    _ => {
                        let static_string = extract_static_string(&*arg0.expr);
                        if let Some(static_string) = static_string {
                            message_text = Some(static_string);
                        } else {
                            // Dynamic expression (Identifier, CallExpression, BinaryExpression,
                            // etc.)
                            warn_dynamic_expression(&*arg0.expr);
                        }
                    }
                }
            }
        }

        call.visit_mut_children_with(self);
    }

    fn visit_mut_module(&mut self, module: &mut Module) {
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

    fn visit_mut_var_declarator(&mut self, node: &mut VarDeclarator) {
        if let Some(name) = node.name.as_ident() {
            let mut call_expr = None;

            // Handle direct CallExpression: const t = useExtracted();

            if let Some(init) = &mut node.init {
                match &mut **init {
                    Expr::Call(init_call) => {
                        if let Callee::Expr(box Expr::Ident(callee)) = &init_call.callee {
                            if self.hook_local_name == Some(callee.to_id()) {
                                init_call.callee =
                                    Callee::Expr(self.hook_local_name.clone().unwrap().into());
                                call_expr = Some(init_call);
                            }
                        }
                    }

                    Expr::Await(AwaitExpr {
                        arg:
                            box Expr::Call(
                                arg @ CallExpr {
                                    callee: Callee::Expr(box Expr::Ident(callee)),
                                    ..
                                },
                            ),
                        ..
                    }) => {
                        if self.hook_local_name == Some(callee.to_id()) {
                            arg.callee = Callee::Expr(self.hook_local_name.clone().unwrap().into());
                            call_expr = Some(arg);
                        }
                    }

                    _ => {}
                }
            }

            if let Some(call_expr) = call_expr {
                let namespace = call_expr.args.first().and_then(|arg| match &*arg.expr {
                    Expr::Lit(Lit::Str(str)) => Some(str.value.clone()),
                    Expr::Object(ObjectLit { props: props, .. }) => props.iter().find_map(|prop| {
                        let prop = prop.as_prop()?.as_key_value()?;
                        match &prop.key {
                            PropName::Ident(ident) => {
                                if ident.sym == "namespace" {
                                    Some(extract_static_string(&prop.value))
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        }
                    }),
                    _ => None,
                });

                self.define_translator(name.to_id(), namespace)
            }
        }

        node.visit_mut_children_with(self);
    }
}

fn extract_static_string(value: &Expr) -> Option<Wtf8Atom> {
    todo!()
}

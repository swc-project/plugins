use std::{
    collections::{HashMap, HashSet},
    hash::{DefaultHasher, Hasher},
};

use serde::Deserialize;
use swc_atoms::Atom;
use swc_common::DUMMY_SP;
use swc_core::quote;
use swc_ecma_ast::{
    AssignExpr, Callee, ComputedPropName, Expr, ExprOrSpread, Function, Id, Ident, ImportDecl,
    ImportSpecifier, Lit, MemberExpr, ModuleExportName, ObjectPatProp, Pass, Tpl, TplElement,
    VarDeclarator,
};
use swc_ecma_visit::{visit_mut_pass, VisitMut, VisitMutWith};

pub fn swc_prefresh(config: PrefreshPluginConfig, file_hash: String) -> impl Pass {
    visit_mut_pass(PrefreshPlugin::new(config, file_hash))
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrefreshPluginConfig {
    #[serde(default = "default_library")]
    pub library: Vec<String>,
}
fn default_library() -> Vec<String> {
    vec!["preact".into(), "react".into(), "preact/compat".into()]
}
pub fn hash_string(s: &str) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(s.as_bytes());
    let hash_result = hasher.finish();
    format!("{hash_result:x}")
}

#[derive(Debug)]
pub struct PrefreshPlugin {
    config: PrefreshPluginConfig,
    file_hash: String,
    parent_key: String,
    param_key: String,
    counter: HashMap<String, usize>,
    local: HashSet<Id>,
    lib_local: HashSet<Id>,
}

impl PrefreshPlugin {
    pub fn new(config: PrefreshPluginConfig, file_hash: String) -> Self {
        Self {
            config,
            file_hash,
            parent_key: Default::default(),
            param_key: Default::default(),
            counter: Default::default(),
            local: Default::default(),
            lib_local: Default::default(),
        }
    }
}

impl PrefreshPlugin {
    fn is_from_lib(&self, mem: &MemberExpr) -> bool {
        let Some(root) = mem.obj.as_ident() else {
            return false;
        };
        if !self.lib_local.contains(&root.to_id()) {
            return false;
        }

        // xxx["createContext"]
        if mem.prop.is_computed() {
            let Some(ComputedPropName { expr, .. }) = mem.prop.as_computed() else {
                return false;
            };
            let Expr::Lit(Lit::Str(lit)) = expr.as_ref() else {
                return false;
            };
            lit.value == "createContext"
        } else {
            // xxx.createContext
            mem.prop
                .as_ident()
                .is_some_and(|id| id.sym == "createContext")
        }
    }
}

impl VisitMut for PrefreshPlugin {
    fn visit_mut_import_decl(&mut self, import_decl: &mut ImportDecl) {
        let import_from = import_decl.src.value.to_string_lossy().into_owned();
        let is_library = self.config.library.contains(&import_from);

        if !is_library {
            return;
        }

        for spec in &import_decl.specifiers {
            match spec {
                ImportSpecifier::Default(spec) => {
                    self.lib_local.insert(spec.local.to_id());
                }
                ImportSpecifier::Named(spec) => {
                    if let Some(imported) = &spec.imported {
                        let name = match imported {
                            ModuleExportName::Ident(ident) => &ident.sym,
                            ModuleExportName::Str(s) => &s.value.to_atom_lossy(),
                        };
                        if name == "createContext" {
                            self.local.insert(spec.local.to_id());
                        }
                    } else if spec.local.sym == "createContext" {
                        self.local.insert(spec.local.to_id());
                    }
                }
                ImportSpecifier::Namespace(spec) => {
                    self.lib_local.insert(spec.local.to_id());
                }
            }
        }
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        let Expr::Call(call_expr) = expr else {
            expr.visit_mut_children_with(self);
            return;
        };

        let is_create_context = match &call_expr.callee {
            Callee::Expr(expr) => match expr.as_ref() {
                Expr::Ident(id) => self.local.contains(&id.to_id()),
                Expr::Member(mem) => self.is_from_lib(mem),
                _ => false,
            },
            _ => false,
        };

        if !is_create_context {
            call_expr.visit_mut_children_with(self);
            return;
        }

        let mut cnt = *self.counter.entry(self.parent_key.clone()).or_insert(0);
        cnt += 1;
        self.counter.insert(self.parent_key.clone(), cnt);

        let context_id = format!(
            "{}{}{}{}",
            self.file_hash, self.parent_key, cnt, self.param_key
        );

        let parts = context_id.split("_PARAM").collect::<Vec<&str>>();
        let exprs = parts
            .iter()
            .skip(1)
            .map(|s| {
                Box::new(Expr::Ident(Ident {
                    sym: Atom::from(s.replace('}', "").to_string()),
                    ..Default::default()
                }))
            })
            .collect::<Vec<_>>();

        let mut quasis = vec![TplElement {
            span: DUMMY_SP,
            tail: false,
            cooked: None,
            raw: Atom::from(
                parts
                    .first()
                    .expect("Should have at lease on part")
                    .to_string(),
            ),
        }];
        quasis.extend(
            exprs
                .iter()
                .map(|_| TplElement {
                    span: DUMMY_SP,
                    tail: false,
                    cooked: None,
                    raw: Atom::from(""),
                })
                .collect::<Vec<_>>(),
        );

        let create_context_expr = call_expr
            .callee
            .as_expr()
            .expect("Should convert callee to expr")
            .as_ref()
            .clone();
        let ident_expr = Expr::Tpl(Tpl {
            span: DUMMY_SP,
            exprs,
            quasis,
        });

        let replacement = if let Some(ExprOrSpread { expr, spread: None }) = call_expr.args.first()
        {
            quote!(
              "Object.assign(($create_context[$ident] || ($create_context[$ident]=$create_context($value))), {__:$value})" as Expr,
              create_context: Expr = create_context_expr,
              ident: Expr = ident_expr,
              value: Expr = expr.as_ref().clone()
            )
        } else {
            quote!(
              "($create_context[$ident] || ($create_context[$ident]=$create_context()))" as Expr,
              create_context: Expr = create_context_expr,
              ident: Expr = ident_expr,
            )
        };

        *expr = replacement;
    }

    fn visit_mut_object_pat_prop(&mut self, obj_pat_prop: &mut ObjectPatProp) {
        if let Some(key) = obj_pat_prop.as_key_value().and_then(|kv| kv.key.as_str()) {
            let old_key = self.parent_key.clone();
            self.parent_key = format!("__{}", key.value.to_string_lossy());
            obj_pat_prop.visit_mut_children_with(self);
            self.parent_key = old_key;
        } else {
            obj_pat_prop.visit_mut_children_with(self);
        }
    }

    fn visit_mut_var_declarator(&mut self, var_declarator_expr: &mut VarDeclarator) {
        if let Some(id) = var_declarator_expr.name.as_ident() {
            let old_key = self.parent_key.clone();
            self.parent_key = format!("${}", id.sym);
            var_declarator_expr.visit_mut_children_with(self);
            self.parent_key = old_key;
        } else {
            var_declarator_expr.visit_mut_children_with(self);
        }
    }

    fn visit_mut_assign_expr(&mut self, assign_expr: &mut AssignExpr) {
        if let Some(id) = assign_expr.left.as_ident() {
            let old_key = self.parent_key.clone();
            self.parent_key = format!("_{}", id.sym);
            assign_expr.visit_mut_children_with(self);
            self.parent_key = old_key;
        } else {
            assign_expr.visit_mut_children_with(self);
        }
    }

    fn visit_mut_function(&mut self, func: &mut Function) {
        let params = func
            .params
            .iter()
            .filter_map(|p| p.pat.as_ident().map(|id| id.sym.to_string()))
            .collect::<Vec<String>>();

        if params.is_empty() {
            func.visit_mut_children_with(self);
        } else {
            let old_key = self.param_key.clone();
            self.param_key = format!("__PARAM{}", params.join("_PARAM"));
            func.visit_mut_children_with(self);
            self.param_key = old_key;
        }
    }
}

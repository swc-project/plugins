use std::collections::{HashMap, HashSet};

use swc_atoms::Atom;
use swc_common::DUMMY_SP;
use swc_ecma_ast::*;
use swc_ecma_visit::{noop_visit_mut_type, VisitMut, VisitMutWith};

use crate::config::BuildTimeConfig;

/// Build-time transformer that replaces feature flag identifiers with
/// __SWC_FLAGS__ markers
pub struct BuildTimeTransform {
    config: BuildTimeConfig,
    /// Map of flag identifier Id -> flag name
    flag_map: HashMap<Id, String>,
    /// Import sources to remove (library names)
    imports_to_remove: HashSet<Atom>,
    /// Call expressions to remove (span-based tracking)
    /// We track the span of variable declarators that contain flag function
    /// calls
    declarators_to_remove: HashSet<u32>, // Using lo() as unique identifier
}

impl BuildTimeTransform {
    pub fn new(config: BuildTimeConfig) -> Self {
        // Build set of import sources from config
        let imports_to_remove: HashSet<Atom> = config
            .libraries
            .keys()
            .map(|k| Atom::from(k.as_str()))
            .collect();

        Self {
            config,
            flag_map: HashMap::new(),
            imports_to_remove,
            declarators_to_remove: HashSet::new(),
        }
    }

    /// Check if a call expression is a flag function call
    fn is_flag_function_call(&self, callee: &Callee) -> bool {
        if let Callee::Expr(expr) = callee {
            if let Expr::Ident(ident) = &**expr {
                // Check if this identifier name matches any configured function
                return self
                    .config
                    .libraries
                    .values()
                    .any(|lib_config| lib_config.functions.iter().any(|f| f == &*ident.sym));
            }
        }
        false
    }

    /// Analyze a variable declarator to detect flag destructuring
    fn analyze_declarator(&mut self, declarator: &VarDeclarator) {
        // Look for pattern: const { flagA, flagB } = useFlags()
        if let Some(init) = &declarator.init {
            if let Expr::Call(call_expr) = &**init {
                if self.is_flag_function_call(&call_expr.callee) {
                    // This is a flag function call, extract flag names from pattern
                    if let Pat::Object(obj_pat) = &declarator.name {
                        for prop in &obj_pat.props {
                            if let ObjectPatProp::KeyValue(kv) = prop {
                                // Extract the flag name from the key
                                let flag_name = match &kv.key {
                                    PropName::Ident(ident_name) => {
                                        ident_name.sym.as_ref().to_string()
                                    }
                                    PropName::Str(str_name) => {
                                        // Convert Wtf8Atom to String
                                        str_name
                                            .value
                                            .as_str()
                                            .map(|s| s.to_string())
                                            .unwrap_or_else(|| {
                                                // If not valid UTF-8, use lossy conversion
                                                str_name.value.to_atom_lossy().to_string()
                                            })
                                    }
                                    _ => continue,
                                };

                                // Skip if excluded
                                if self.config.exclude_flags.contains(&flag_name) {
                                    continue;
                                }

                                // Extract the local binding identifier
                                if let Pat::Ident(binding_ident) = &*kv.value {
                                    let flag_id = binding_ident.id.to_id();
                                    self.flag_map.insert(flag_id, flag_name);
                                }
                            } else if let ObjectPatProp::Assign(assign_prop) = prop {
                                // Shorthand: { flagA } = useFlags()
                                let flag_name = assign_prop.key.sym.to_string();

                                // Skip if excluded
                                if self.config.exclude_flags.contains(&flag_name) {
                                    continue;
                                }

                                let flag_id = assign_prop.key.to_id();
                                self.flag_map.insert(flag_id, flag_name);
                            }
                        }
                    }

                    // Mark this declarator for removal
                    self.declarators_to_remove.insert(declarator.span.lo.0);
                }
            }
        }
    }

    /// Create a __SWC_FLAGS__.flagName member expression
    fn create_flag_member_expr(&self, flag_name: &str) -> Expr {
        Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(Ident::new(
                Atom::from(self.config.marker_object.as_str()),
                DUMMY_SP,
                Default::default(),
            ))),
            prop: MemberProp::Ident(IdentName {
                span: DUMMY_SP,
                sym: Atom::from(flag_name),
            }),
        })
    }
}

impl VisitMut for BuildTimeTransform {
    noop_visit_mut_type!();

    fn visit_mut_module(&mut self, module: &mut Module) {
        // First pass: analyze entire module to build flag map
        // We need to walk the whole tree to find all var declarations
        for item in &module.body {
            self.analyze_module_item(item);
        }

        // Second pass: transform (visit children)
        module.visit_mut_children_with(self);

        // Third pass: remove imports from configured libraries
        module.body.retain(|item| {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = item {
                // Remove if the source matches a configured library
                return !self.imports_to_remove.contains(&import_decl.src.value);
            }
            true
        });
    }

    fn visit_mut_script(&mut self, script: &mut Script) {
        // First pass: analyze to build flag map
        for stmt in &script.body {
            self.analyze_stmt(stmt);
        }

        // Second pass: transform
        script.visit_mut_children_with(self);
    }

    fn visit_mut_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        // Visit children first to transform expressions
        stmts.visit_mut_children_with(self);

        // Remove statements that are flag function calls
        stmts.retain(|stmt| {
            if let Stmt::Decl(Decl::Var(var_decl)) = stmt {
                // Keep the statement only if none of its declarators should be removed
                return !var_decl
                    .decls
                    .iter()
                    .any(|decl| self.declarators_to_remove.contains(&decl.span.lo.0));
            }
            true
        });
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        // Visit children first
        expr.visit_mut_children_with(self);

        // Replace flag identifiers with __SWC_FLAGS__.flagName
        if let Expr::Ident(ident) = expr {
            if let Some(flag_name) = self.flag_map.get(&ident.to_id()) {
                *expr = self.create_flag_member_expr(flag_name);
            }
        }
    }
}

impl BuildTimeTransform {
    /// Recursively analyze a module item to find var declarations
    fn analyze_module_item(&mut self, item: &ModuleItem) {
        match item {
            ModuleItem::Stmt(stmt) => self.analyze_stmt(stmt),
            _ => {}
        }
    }

    /// Recursively analyze a statement to find var declarations
    fn analyze_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Decl(Decl::Var(var_decl)) => {
                for declarator in &var_decl.decls {
                    self.analyze_declarator(declarator);
                }
            }
            Stmt::Decl(Decl::Fn(fn_decl)) => {
                if let Some(body) = &fn_decl.function.body {
                    for stmt in &body.stmts {
                        self.analyze_stmt(stmt);
                    }
                }
            }
            Stmt::Block(block_stmt) => {
                for stmt in &block_stmt.stmts {
                    self.analyze_stmt(stmt);
                }
            }
            Stmt::If(if_stmt) => {
                self.analyze_stmt(&if_stmt.cons);
                if let Some(alt) = &if_stmt.alt {
                    self.analyze_stmt(alt);
                }
            }
            Stmt::While(while_stmt) => {
                self.analyze_stmt(&while_stmt.body);
            }
            Stmt::For(for_stmt) => {
                self.analyze_stmt(&for_stmt.body);
            }
            Stmt::ForIn(for_in_stmt) => {
                self.analyze_stmt(&for_in_stmt.body);
            }
            Stmt::ForOf(for_of_stmt) => {
                self.analyze_stmt(&for_of_stmt.body);
            }
            _ => {}
        }
    }
}

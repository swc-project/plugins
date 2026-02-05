use std::collections::{HashMap, HashSet};

use swc_atoms::Atom;
use swc_common::DUMMY_SP;
use swc_ecma_ast::*;
use swc_ecma_visit::{noop_visit_mut_type, VisitMut, VisitMutWith};

use crate::config::BuildTimeConfig;

/// Information about a flag object variable (e.g., `const flags = useFlags()`)
struct FlagObjectInfo {
    span_lo: u32, // For removal tracking
}

/// Build-time transformer that replaces feature flag identifiers with
/// __SWC_FLAGS__ markers
pub struct BuildTimeTransform {
    config: BuildTimeConfig,
    /// Map of flag identifier Id -> flag name
    flag_map: HashMap<Id, String>,
    /// Map of flag object variable Id -> info (for tracking `const flags =
    /// useFlags()`)
    flag_object_map: HashMap<Id, FlagObjectInfo>,
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
            flag_object_map: HashMap::new(),
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

    /// Extract flags from an object pattern and add them to flag_map
    /// Returns true if any flags were extracted
    fn extract_flags_from_object_pattern(&mut self, obj_pat: &ObjectPat) -> bool {
        let mut extracted_any = false;

        for prop in &obj_pat.props {
            if let ObjectPatProp::KeyValue(kv) = prop {
                // Extract the flag name from the key
                let flag_name = match &kv.key {
                    PropName::Ident(ident_name) => ident_name.sym.as_ref().to_string(),
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
                    extracted_any = true;
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
                extracted_any = true;
            }
        }

        extracted_any
    }

    /// Analyze a variable declarator to detect flag destructuring
    fn analyze_declarator(&mut self, declarator: &VarDeclarator) {
        // Pattern 1: const { flagA, flagB } = useFlags()
        if let Some(init) = &declarator.init {
            if let Expr::Call(call_expr) = &**init {
                if self.is_flag_function_call(&call_expr.callee) {
                    match &declarator.name {
                        // Direct destructuring from flag function call
                        Pat::Object(obj_pat) => {
                            let extracted = self.extract_flags_from_object_pattern(obj_pat);
                            // Only remove if we actually extracted flags
                            if extracted {
                                self.declarators_to_remove.insert(declarator.span.lo.0);
                            }
                        }
                        // Pattern 2: const flags = useFlags()
                        Pat::Ident(ident) => {
                            let flag_id = ident.id.to_id();
                            self.flag_object_map.insert(
                                flag_id,
                                FlagObjectInfo {
                                    span_lo: declarator.span.lo.0,
                                },
                            );
                            // Don't remove yet - we'll remove only if flags are
                            // used
                        }
                        _ => {}
                    }
                    return;
                }
            }
        }

        // Pattern 3: const { flagA } = flags (indirect destructuring)
        if let Pat::Object(obj_pat) = &declarator.name {
            if let Some(init) = &declarator.init {
                if let Expr::Ident(ident) = &**init {
                    let source_id = ident.to_id();
                    // Get span_lo before calling extract method to avoid borrow checker issues
                    let source_span_lo = self
                        .flag_object_map
                        .get(&source_id)
                        .map(|info| info.span_lo);

                    if let Some(source_span) = source_span_lo {
                        let extracted = self.extract_flags_from_object_pattern(obj_pat);
                        // Only remove if we actually extracted flags
                        if extracted {
                            // Remove both the destructuring declaration and the source flag object
                            self.declarators_to_remove.insert(declarator.span.lo.0);
                            self.declarators_to_remove.insert(source_span);
                        }
                    }
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

        // Handle member expressions: flags.featureA → __SWC_FLAGS__.featureA
        if let Expr::Member(member_expr) = expr {
            if let Expr::Ident(obj_ident) = &*member_expr.obj {
                let obj_id = obj_ident.to_id();

                if let Some(info) = self.flag_object_map.get(&obj_id) {
                    if let MemberProp::Ident(prop_ident) = &member_expr.prop {
                        let flag_name = prop_ident.sym.to_string();

                        // Skip if excluded
                        if !self.config.exclude_flags.contains(&flag_name) {
                            // Mark the flag object declaration for removal since we're transforming
                            // this usage
                            self.declarators_to_remove.insert(info.span_lo);
                            *expr = self.create_flag_member_expr(&flag_name);
                        }
                    }
                }
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

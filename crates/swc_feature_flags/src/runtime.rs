use swc_common::{util::take::Take, DUMMY_SP};
use swc_ecma_ast::*;
use swc_ecma_visit::{noop_visit_mut_type, VisitMut, VisitMutWith};

use crate::{
    config::RuntimeConfig,
    stats::{estimate_expr_size, estimate_stmt_size, TransformStats},
};

/// Runtime transformer that substitutes feature flag values and eliminates dead
/// code
pub struct RuntimeTransform {
    config: RuntimeConfig,
    stats: TransformStats,
}

impl RuntimeTransform {
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            config,
            stats: TransformStats::new(),
        }
    }

    /// Get statistics collected during transformation
    pub fn stats(&self) -> &TransformStats {
        &self.stats
    }

    /// Take ownership of statistics
    pub fn take_stats(self) -> TransformStats {
        self.stats
    }

    /// Check if an expression is __SWC_FLAGS__.flagName and return the flag
    /// name
    fn extract_flag_name(&self, expr: &Expr) -> Option<String> {
        if let Expr::Member(member_expr) = expr {
            // Check if object is the marker identifier
            if let Expr::Ident(obj_ident) = &*member_expr.obj {
                if obj_ident.sym == self.config.marker_object {
                    // Extract the flag name from the property
                    if let MemberProp::Ident(prop_ident) = &member_expr.prop {
                        return Some(prop_ident.sym.to_string());
                    }
                }
            }
        }
        None
    }

    /// Get the boolean value for a flag name
    fn get_flag_value(&mut self, flag_name: &str) -> Option<bool> {
        if let Some(&value) = self.config.flag_values.get(flag_name) {
            if self.config.collect_stats {
                self.stats.record_flag_processed(flag_name.to_string());
            }
            Some(value)
        } else {
            None
        }
    }

    /// Check if an expression is a literal boolean
    fn is_literal_bool(expr: &Expr) -> Option<bool> {
        if let Expr::Lit(Lit::Bool(Bool { value, .. })) = expr {
            Some(*value)
        } else {
            None
        }
    }

    /// Simplify logical expressions after flag substitution
    fn simplify_logical_expr(&mut self, expr: &mut Expr) {
        // Calculate old size first if we need stats
        let old_size = if self.config.collect_stats {
            estimate_expr_size(expr)
        } else {
            0
        };

        let mut modified = false;

        if let Expr::Bin(bin_expr) = expr {
            match bin_expr.op {
                // true && right => right
                // false && right => false
                BinaryOp::LogicalAnd => {
                    if let Some(left_val) = Self::is_literal_bool(&bin_expr.left) {
                        if left_val {
                            // true && right => right
                            *expr = (*bin_expr.right).take();
                        } else {
                            // false && right => false
                            *expr = Expr::Lit(Lit::Bool(Bool {
                                span: DUMMY_SP,
                                value: false,
                            }));
                        }
                        modified = true;
                    } else if let Some(right_val) = Self::is_literal_bool(&bin_expr.right) {
                        if right_val {
                            // left && true => left
                            *expr = (*bin_expr.left).take();
                        } else {
                            // left && false => false
                            *expr = Expr::Lit(Lit::Bool(Bool {
                                span: DUMMY_SP,
                                value: false,
                            }));
                        }
                        modified = true;
                    }
                }
                // true || right => true
                // false || right => right
                BinaryOp::LogicalOr => {
                    if let Some(left_val) = Self::is_literal_bool(&bin_expr.left) {
                        if left_val {
                            // true || right => true
                            *expr = Expr::Lit(Lit::Bool(Bool {
                                span: DUMMY_SP,
                                value: true,
                            }));
                        } else {
                            // false || right => right
                            *expr = (*bin_expr.right).take();
                        }
                        modified = true;
                    } else if let Some(right_val) = Self::is_literal_bool(&bin_expr.right) {
                        if right_val {
                            // left || true => true
                            *expr = Expr::Lit(Lit::Bool(Bool {
                                span: DUMMY_SP,
                                value: true,
                            }));
                        } else {
                            // left || false => left
                            *expr = (*bin_expr.left).take();
                        }
                        modified = true;
                    }
                }
                _ => {}
            }
        } else if let Expr::Unary(unary_expr) = expr {
            // !true => false, !false => true
            if unary_expr.op == UnaryOp::Bang {
                if let Some(val) = Self::is_literal_bool(&unary_expr.arg) {
                    *expr = Expr::Lit(Lit::Bool(Bool {
                        span: DUMMY_SP,
                        value: !val,
                    }));
                    modified = true;
                }
            }
        }

        // Record stats after modification
        if modified && self.config.collect_stats {
            let new_size = estimate_expr_size(expr);
            self.stats
                .record_expr_elimination(old_size.saturating_sub(new_size));
        }
    }
}

impl VisitMut for RuntimeTransform {
    noop_visit_mut_type!();

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        // Visit children first to handle nested expressions
        expr.visit_mut_children_with(self);

        // Replace __SWC_FLAGS__.flagName with literal boolean
        if let Some(flag_name) = self.extract_flag_name(expr) {
            if let Some(value) = self.get_flag_value(&flag_name) {
                *expr = Expr::Lit(Lit::Bool(Bool {
                    span: DUMMY_SP,
                    value,
                }));
            }
        }

        // Simplify ternary expressions: flag ? a : b
        // Check first if we can simplify, calculate size before modifying
        let can_simplify_ternary = matches!(expr, Expr::Cond(cond_expr) if Self::is_literal_bool(&cond_expr.test).is_some());
        let old_size = if can_simplify_ternary && self.config.collect_stats {
            estimate_expr_size(expr)
        } else {
            0
        };

        if let Expr::Cond(cond_expr) = expr {
            if let Some(test_val) = Self::is_literal_bool(&cond_expr.test) {
                *expr = if test_val {
                    (*cond_expr.cons).take()
                } else {
                    (*cond_expr.alt).take()
                };
                if self.config.collect_stats {
                    let new_size = estimate_expr_size(expr);
                    self.stats
                        .record_expr_elimination(old_size.saturating_sub(new_size));
                }
            }
        }

        // Simplify logical expressions
        self.simplify_logical_expr(expr);
    }

    fn visit_mut_stmt(&mut self, stmt: &mut Stmt) {
        // Visit children first
        stmt.visit_mut_children_with(self);

        // Eliminate if statements with constant conditions
        // Check first if we can simplify, calculate size before modifying
        let can_simplify_if =
            matches!(stmt, Stmt::If(if_stmt) if Self::is_literal_bool(&if_stmt.test).is_some());
        let old_size = if can_simplify_if && self.config.collect_stats {
            estimate_stmt_size(stmt)
        } else {
            0
        };

        if let Stmt::If(if_stmt) = stmt {
            if let Some(test_val) = Self::is_literal_bool(&if_stmt.test) {
                if test_val {
                    // if (true) { cons } else { alt } => cons
                    *stmt = *if_stmt.cons.take();
                } else if let Some(alt) = &mut if_stmt.alt {
                    // if (false) { cons } else { alt } => alt
                    *stmt = *alt.take();
                } else {
                    // if (false) { cons } => empty
                    *stmt = Stmt::Empty(EmptyStmt { span: DUMMY_SP });
                }

                if self.config.collect_stats {
                    let new_size = estimate_stmt_size(stmt);
                    self.stats
                        .record_branch_elimination(stmt, old_size.saturating_sub(new_size));
                }
            }
        }
    }

    fn visit_mut_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        // Visit children first
        stmts.visit_mut_children_with(self);

        // Flatten block statements and remove empty statements
        let mut new_stmts = Vec::with_capacity(stmts.len());
        for stmt in stmts.take() {
            match stmt {
                // Remove empty statements that were created by DCE
                Stmt::Empty(_) => {}
                // Flatten block statements (unwrap single blocks)
                Stmt::Block(block_stmt) => {
                    // If this is a block statement at the top level of a statement list,
                    // unwrap it and add its contents directly
                    new_stmts.extend(block_stmt.stmts);
                }
                _ => new_stmts.push(stmt),
            }
        }
        *stmts = new_stmts;
    }
}

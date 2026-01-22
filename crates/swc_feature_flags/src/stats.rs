use std::collections::HashSet;

use swc_ecma_ast::{Callee, Decl, Expr, Lit, Stmt};

/// Statistics collected during transformation
#[derive(Clone, Debug, Default)]
pub struct TransformStats {
    /// Original code size in bytes (approximate)
    pub original_bytes: usize,

    /// Bytes removed during transformation (approximate)
    pub removed_bytes: usize,

    /// Number of branches eliminated
    pub branches_eliminated: usize,

    /// Flags that were processed
    pub flags_processed: HashSet<String>,
}

impl TransformStats {
    /// Create a new empty stats collector
    pub fn new() -> Self {
        Self::default()
    }

    /// Record that a branch was eliminated
    pub fn record_branch_elimination(&mut self, _original: &Stmt, removed_size: usize) {
        self.branches_eliminated += 1;
        self.removed_bytes += removed_size;
    }

    /// Record that an expression was eliminated
    pub fn record_expr_elimination(&mut self, removed_size: usize) {
        self.removed_bytes += removed_size;
    }

    /// Record that a flag was processed
    pub fn record_flag_processed(&mut self, flag_name: String) {
        self.flags_processed.insert(flag_name);
    }

    /// Get the percentage of bytes removed
    pub fn bytes_removed_percent(&self) -> f64 {
        if self.original_bytes == 0 {
            0.0
        } else {
            (self.removed_bytes as f64 / self.original_bytes as f64) * 100.0
        }
    }
}

/// Estimate the size of a statement in bytes (for statistics)
/// This is a simple approximation based on node type and child count
pub fn estimate_stmt_size(stmt: &Stmt) -> usize {
    match stmt {
        Stmt::Empty(_) => 1,
        Stmt::Expr(e) => estimate_expr_size(&e.expr) + 2, // expr + ';'
        Stmt::If(if_stmt) => {
            let mut size = 4; // 'if' + '(' + ')' + braces
            size += estimate_expr_size(&if_stmt.test);
            size += estimate_stmt_size(&if_stmt.cons);
            if let Some(alt) = &if_stmt.alt {
                size += 4; // 'else'
                size += estimate_stmt_size(alt);
            }
            size
        }
        Stmt::Block(block) => {
            let mut size = 2; // '{' + '}'
            for stmt in &block.stmts {
                size += estimate_stmt_size(stmt);
            }
            size
        }
        Stmt::Return(ret) => {
            let mut size = 6; // 'return' + ';'
            if let Some(arg) = &ret.arg {
                size += estimate_expr_size(arg);
            }
            size
        }
        Stmt::Decl(decl) => {
            match decl {
                Decl::Var(var_decl) => {
                    let mut size = 4; // 'var' or 'let' or 'const'
                    for declarator in &var_decl.decls {
                        size += 20; // Approximate size of declarator
                        if let Some(init) = &declarator.init {
                            size += estimate_expr_size(init);
                        }
                    }
                    size
                }
                _ => 50, // Rough estimate for other declarations
            }
        }
        _ => 50, // Rough estimate for other statement types
    }
}

/// Estimate the size of an expression in bytes (for statistics)
/// This is a simple approximation based on node type and child count
pub fn estimate_expr_size(expr: &Expr) -> usize {
    match expr {
        Expr::Lit(lit) => match lit {
            Lit::Str(s) => s.value.len() + 2, // string + quotes
            Lit::Bool(_) => 5,                // 'true' or 'false'
            Lit::Null(_) => 4,                // 'null'
            Lit::Num(_) => 10,                // approximate
            Lit::BigInt(_) => 15,             // approximate
            Lit::Regex(_) => 20,              // approximate
            Lit::JSXText(_) => 20,            // approximate
        },
        Expr::Ident(_) => 10, // average identifier length
        Expr::Member(member) => {
            estimate_expr_size(&member.obj) + 5 // object + '.' + property
        }
        Expr::Bin(bin) => {
            estimate_expr_size(&bin.left) + estimate_expr_size(&bin.right) + 3 // left + op + right
        }
        Expr::Unary(unary) => {
            estimate_expr_size(&unary.arg) + 2 // op + arg
        }
        Expr::Cond(cond) => {
            estimate_expr_size(&cond.test)
                + estimate_expr_size(&cond.cons)
                + estimate_expr_size(&cond.alt)
                + 4 // test + '?' + cons + ':' + alt
        }
        Expr::Call(call) => {
            let mut size = estimate_callee_size(&call.callee) + 2; // callee + '(' + ')'
            for arg in &call.args {
                size += estimate_expr_size(&arg.expr) + 1; // arg + ','
            }
            size
        }
        Expr::Array(arr) => {
            let mut size = 2; // '[' + ']'
            for elem in &arr.elems {
                if let Some(elem) = elem {
                    size += estimate_expr_size(&elem.expr) + 1; // elem + ','
                }
            }
            size
        }
        Expr::Object(obj) => {
            let mut size = 2; // '{' + '}'
            for _prop in &obj.props {
                size += 20; // Rough estimate for property
            }
            size
        }
        _ => 30, // Rough estimate for other expression types
    }
}

fn estimate_callee_size(callee: &Callee) -> usize {
    match callee {
        Callee::Expr(expr) => estimate_expr_size(expr),
        Callee::Super(_) => 5,  // 'super'
        Callee::Import(_) => 6, // 'import'
    }
}

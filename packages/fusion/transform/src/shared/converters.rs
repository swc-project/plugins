use std::collections::HashMap;
use swc_core::ecma::{utils::quote_ident, ast::Ident};

#[derive(Debug)]
pub struct JsVarConverter {
    map: HashMap<String, String>,
    counter: u32,
    prefix: String
}

impl JsVarConverter {
    pub fn new(prefix: &str) -> Self {
        Self {
            map: HashMap::new(),
            counter: 0,
            prefix: prefix.to_string()
        }
    }

    pub fn ident_from_path(&mut self, s: &str) -> Ident {
        if let Some(js_var) = self.map.get(s) {
            // If this string was already processed, reuse the old result.
            return quote_ident!(format!("$_{}_{}", self.prefix, js_var));
        }

        let mut js_var = String::with_capacity(s.len());
        let chars: Vec<char> = s.chars().collect();

        // Check the first character. If it's a digit, prepend an underscore.
        if chars[0].is_numeric() {
            js_var.push('_');
        }

        // For the rest of the characters, replace non-alphanumeric characters with underscores.
        for ch in chars {
            if ch.is_alphanumeric() {
                js_var.push(ch);
            } else {
                js_var.push('_');
            }
        }

        // Append the counter and then increase it for the next variable.
        js_var.push_str(&self.counter.to_string());
        self.counter += 1;

        // Store the result in the map for possible reuse.
        self.map.insert(s.to_string(), js_var.clone());

        return quote_ident!(format!("$_{}_{}", self.prefix, js_var));
    }
}

use std::cell::RefCell;

use swc_core::{
    common::{collections::AHashMap, SyntaxContext},
    ecma::ast::*,
};

pub use self::analyzer::{analyzer};

mod analyzer;

/// This is created once per file.
#[derive(Debug, Default)]
pub struct State {

}

impl State {
}

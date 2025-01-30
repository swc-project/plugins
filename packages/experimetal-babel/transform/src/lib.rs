use serde::{Deserialize, Serialize};
use swc_ecma_ast::Program;

pub struct Config {}

pub struct Transform<'a> {
    pub transform_code: &'a str,
    pub config: &'a Config,
}

#[derive(Serialize, Deserialize)]
pub struct TransformOutput {
    pub code: String,
    #[serde(default)]
    pub map: Option<String>,
}

impl Transform<'_> {
    pub fn apply(&self, program: &Program) -> Program {
        todo!()
    }
}

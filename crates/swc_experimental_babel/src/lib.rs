use anyhow::{Context as _, Result};
use rquickjs::{Function, IntoJs};
use serde::{Deserialize, Serialize};
use swc_common::{sync::Lrc, SourceMap, SourceMapper};
use swc_ecma_ast::{Program, SourceMapperExt};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter, Node};

use crate::qjs::with_quickjs_context;

mod qjs;

pub struct Config {}

pub struct Transform<'a, S>
where
    S: SourceMapper + SourceMapperExt,
{
    pub transform_code: &'a str,
    pub config: &'a Config,
    cm: Lrc<S>,
}

#[derive(Serialize, Deserialize)]
pub struct TransformOutput {
    pub code: String,
    #[serde(default)]
    pub map: Option<String>,
}

impl TransformOutput {
    pub fn from_swc<S>(cm: Lrc<S>, program: &Program) -> Result<Self>
    where
        S: SourceMapper + SourceMapperExt,
    {
        let dummy_cm = Lrc::new(SourceMap::default());
        let mut buf = vec![];

        {
            let mut wr = JsWriter::new(dummy_cm, "\n", &mut buf, None);
            let mut emitter = Emitter {
                cfg: Default::default(),
                cm,
                comments: None,
                wr: &mut wr,
            };

            program.emit_with(&mut emitter)?;
        }

        Ok(Self {
            code: String::from_utf8(buf).context("failed to convert the generated code to utf8")?,
            map: None,
        })
    }

    pub fn parse(&self) -> Program {
        todo!()
    }
}

impl<S> Transform<'_, S>
where
    S: SourceMapper + SourceMapperExt,
{
    pub fn apply(&self, program: &Program) -> Result<Program> {
        let input = TransformOutput::from_swc(self.cm.clone(), program)?;
        let output = self.apply_transform(input)?;

        Ok(output.parse())
    }

    fn apply_transform(&self, input: TransformOutput) -> Result<TransformOutput> {
        with_quickjs_context(|ctx| {
            let function: Function = ctx
                .eval(self.transform_code)
                .context("failed to evaluate the transform code")?;

            let output = function
                .call(input)
                .context("failed to call the transform function")?;

            Ok(output)
        })
    }
}

impl<'a> IntoJs<'a> for TransformOutput {
    fn into_js(self, ctx: &rquickjs::Ctx<'a>) -> rquickjs::Result<rquickjs::Value<'a>> {
        let obj = rquickjs::Object::new(ctx.clone())?;

        obj.set("code", self.code);
        obj.set("map", self.map);

        obj.into_js(ctx)
    }
}

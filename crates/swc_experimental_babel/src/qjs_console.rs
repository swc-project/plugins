use rquickjs::{class::Trace, Ctx, JsLifetime, Result};

pub fn init(ctx: &Ctx<'_>) -> Result<()> {
    ctx.globals().set("console", Console {})?;
    Ok(())
}

#[derive(Trace, JsLifetime)]
#[rquickjs::class(rename_all = "camelCase")]
struct Console {}

#[rquickjs::methods]
impl Console {
    fn log(&self, message: String) {
        println!("[vm] {message}");
    }
}

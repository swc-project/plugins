use once_cell::unsync::Lazy;
use rquickjs::{Context, Runtime};

thread_local! {
    static QUICKJS_RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("failed to create quickjs runtime"));
}

thread_local! {
    static QUICKJS_CONTEXT: Lazy<Context> = Lazy::new(|| {
        QUICKJS_RUNTIME.with(|rt| Context::full(rt).expect("failed to create context"))
    });
}

pub fn with_quickjs_context<F, R>(f: F) -> R
where
    F: FnOnce(rquickjs::Ctx) -> R,
{
    QUICKJS_CONTEXT.with(|ctx| ctx.with(f))
}

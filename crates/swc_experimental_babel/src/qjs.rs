use once_cell::unsync::Lazy;
use rquickjs::{Context, Runtime};

use crate::qjs_console;

thread_local! {
    static QUICKJS_RUNTIME: Lazy<Runtime> = Lazy::new(new_runtime);
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
    QUICKJS_CONTEXT.with(|ctx| {
        ctx.with(|ctx| {
            qjs_console::init(&ctx).unwrap();
            f(ctx)
        })
    })
}

fn new_runtime() -> Runtime {
    let rt = Runtime::new().expect("failed to create quickjs runtime");
    rt.set_max_stack_size(4 * 1024 * 1024);
    rt
}

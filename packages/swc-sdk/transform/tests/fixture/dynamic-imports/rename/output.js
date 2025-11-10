/*#__DYNAMIC__*/ export async function work() {
    if (process.env.NODE_ENV !== "production") {
        await import("./logger").then(async (module)=>module.debug("Starting work"));
    }
    if (process.env.NODE_ENV !== "production") {
        import("./logger").then(async (module)=>module.debug("Without await"));
    }
}
export function sync() {
    if (process.env.NODE_ENV !== "production") {
        import("./logger").then(async (module)=>module.debug("Without async"));
    }
}

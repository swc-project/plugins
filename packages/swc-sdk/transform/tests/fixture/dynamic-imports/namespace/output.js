/*#__DYNAMIC__*/ export async function work() {
    if (process.env.NODE_ENV !== "production") {
        await import("./logger").then(async (module)=>module.log("Starting work"));
    }
    if (process.env.NODE_ENV !== "production") {
        import("./logger").then(async (module)=>module.log("Without await"));
    }
}
export function sync() {
    if (process.env.NODE_ENV !== "production") {
        import("./logger").then(async (module)=>module.log("Without async"));
    }
}

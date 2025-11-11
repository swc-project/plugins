/*#__DYNAMIC__*/ import { lazy as lazy } from "react";
let LazyComponent;
export async function App() {
    if (process.env.NODE_ENV !== "production") {
        LazyComponent ??= lazy(()=>import("./stripped").then(async (module)=>({
                    default: module.Component
                })));
        return <LazyComponent/>;
    }
    return <></>;
}

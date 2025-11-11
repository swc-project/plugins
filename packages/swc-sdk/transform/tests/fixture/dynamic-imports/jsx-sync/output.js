/*#__DYNAMIC__*/ import { lazy as lazy } from "react";
let LazyComponent;
export function App() {
    if (process.env.NODE_ENV !== "production") {
        LazyComponent ??= lazy(()=>import("./stripped").then(async (module)=>module.Component));
        return <LazyComponent/>;
    }
    return <></>;
}

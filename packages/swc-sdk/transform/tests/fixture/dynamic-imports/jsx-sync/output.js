/*#__DYNAMIC__*/ import { lazy as lazy } from "react";
export function App() {
    if (process.env.NODE_ENV !== "production") {
        const LazyComponent = lazy(()=>import("./stripped").then(async (module)=>module.Component));
        return <LazyComponent/>;
    }
    return <></>;
}

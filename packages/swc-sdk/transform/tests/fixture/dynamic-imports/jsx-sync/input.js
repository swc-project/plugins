/*#__DYNAMIC__*/
import { Component } from "./stripped";

export function App() {
    if (process.env.NODE_ENV !== "production") {
        return <Component />;
    }
    return <></>;
}

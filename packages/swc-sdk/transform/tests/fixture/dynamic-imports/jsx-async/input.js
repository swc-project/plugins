/*#__DYNAMIC__*/
import { Component } from "./stripped";

export async function App() {
    if (process.env.NODE_ENV !== "production") {
        return <Component />;
    }
    return <></>;
}

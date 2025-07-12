export { };
Object.defineProperty(exports, "App", {
    enumerable: true,
    get () {
        return App;
    },
    set (v) {
        App = v;
    },
    configurable: true
});
import React from "react";
const App = ()=>{
    return React.createElement("main", {
        className: "akari"
    }, "Hello", "swc", "plugin");
};

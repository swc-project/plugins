export { };
Object.defineProperty(exports, "A", {
    enumerable: true,
    get () {
        return A;
    },
    set (v) {
        A = v;
    },
    configurable: true
});
Object.defineProperty(exports, "default", {
    enumerable: true,
    get () {
        return _default;
    },
    set (v) {
        _default = v;
    },
    configurable: true
});
import React from "react";
const A = ()=>{
    return <div>real a</div>;
};
const B = ()=>{
    return <exports.A/>;
};
const _default = B;

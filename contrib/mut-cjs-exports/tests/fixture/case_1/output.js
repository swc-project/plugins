export { };
Object.defineProperty(exports, "callChild", {
    enumerable: true,
    get () {
        return callChild;
    },
    set (v) {
        callChild = v;
    },
    configurable: true
});
Object.defineProperty(exports, "child", {
    enumerable: true,
    get () {
        return child;
    },
    set (v) {
        child = v;
    },
    configurable: true
});
const child = ()=>{
    console.log("Hello World!");
};
const callChild = ()=>{
    (0, exports.child)();
};

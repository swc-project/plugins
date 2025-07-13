export { };
Object.defineProperty(exports, "bar", {
    enumerable: true,
    get () {
        return bar;
    },
    set (v) {
        bar = v;
    },
    configurable: true
});
Object.defineProperty(exports, "foo", {
    enumerable: true,
    get () {
        return foo;
    },
    set (v) {
        foo = v;
    },
    configurable: true
});
function foo() {
    foo = ()=>1;
    foo.bar = ()=>2;
    return 3;
}
let bar = function() {
    bar = ()=>1;
    exports.bar.bar = ()=>(0, exports.bar)();
    return 3;
};

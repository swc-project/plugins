export { };
Object.defineProperty(exports, "a", {
    enumerable: true,
    get () {
        return a;
    },
    set (v) {
        a = v;
    },
    configurable: true
});
Object.defineProperty(exports, "b", {
    enumerable: true,
    get () {
        return b;
    },
    set (v) {
        b = v;
    },
    configurable: true
});
Object.defineProperty(exports, "c", {
    enumerable: true,
    get () {
        return c;
    },
    set (v) {
        c = v;
    },
    configurable: true
});
let a = function() {};
function b() {}
class c {
}
(0, exports.a)();
b();
new c();
let _ = {
    a: exports.a,
    b,
    c
};
a = function() {};
b = function() {};
(0, exports.a)``;
b``;

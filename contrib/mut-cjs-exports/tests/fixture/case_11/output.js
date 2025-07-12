export { };
Object.defineProperty(exports, "bar", {
    enumerable: true,
    get () {
        return bar1;
    },
    set (v) {
        bar1 = v;
    },
    configurable: true
});
Object.defineProperty(exports, "foo", {
    enumerable: true,
    get () {
        return foo1;
    },
    set (v) {
        foo1 = v;
    },
    configurable: true
});
Object.defineProperty(exports, "foofoo", {
    enumerable: true,
    get () {
        return foofoo;
    },
    set (v) {
        foofoo = v;
    },
    configurable: true
});
Object.defineProperty(exports, "ns", {
    enumerable: true,
    get () {
        return ns;
    },
    set (v) {
        ns = v;
    },
    configurable: true
});
Object.defineProperty(exports, "y-1", {
    enumerable: true,
    get () {
        return x1;
    },
    set (v) {
        x1 = v;
    },
    configurable: true
});
import { foo } from "foo";
import { foo as foo1, foofoo as foofoo } from "foo";
import { bar } from "bar";
import { "b-a-r" as bar1 } from "bar";
import { "x-1" as x1 } from "baz";
import * as ns from "ns";

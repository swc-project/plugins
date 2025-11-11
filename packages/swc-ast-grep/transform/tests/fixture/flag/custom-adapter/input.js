import { flag } from "@swc/sdk/flag";

const foo = flag({
    decide: () => false,
});

console.log(foo);
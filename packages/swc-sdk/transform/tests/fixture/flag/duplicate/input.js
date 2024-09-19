import { flag } from "@swc/sdk/flag";

const foo = flag({
    decide: () => false,
    key: 'custom'
});

console.log(foo);
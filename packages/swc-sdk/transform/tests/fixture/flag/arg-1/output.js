import { flag } from "@swc/sdk/flag";
const foo = flag({
    decide: ()=>false,
    key: "foo"
});
console.log(foo);

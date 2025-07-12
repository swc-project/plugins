import { foo } from "foo";
export { foo, foofoo } from "foo";

import { bar } from "bar";
export { "b-a-r" as bar } from "bar";

export { "x-1" as "y-1" } from "baz";

export * as ns from "ns";

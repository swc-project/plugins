import { assert, fail } from 'assert';

assert(true, "top level assertion");
fail("should not reach here");

export function shouldRemove() {
  assert(x > 0, "x must be positive");
  const result = compute();
  assert(result !== null, "result cannot be null");
  fail("unreachable");
  return result;
}

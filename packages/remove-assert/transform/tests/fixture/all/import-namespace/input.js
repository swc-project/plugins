import * as assert from 'assert';

assert(true, "top level assertion");

export function shouldRemove() {
  assert(x > 0, "x must be positive");
  const result = compute();
  assert(result !== null, "result cannot be null");
  return result;
}
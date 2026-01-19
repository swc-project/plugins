import assert from 'assert';

assert(true, "direct call");
assert.ok(value, "ok call");
assert.strictEqual(a, b, "strictEqual call");
assert.deepEqual(obj1, obj2, "deepEqual call");

export function shouldRemove() {
  assert(x > 0, "direct call in function");
  assert.ok(x, "ok call in function");
  const result = compute();
  assert.strictEqual(result, expected, "strictEqual call in function");
  return result;
}

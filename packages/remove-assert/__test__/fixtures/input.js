import assert from 'assert';

assert(true, "top level assert");

export function divide(a, b) {
  assert(b !== 0, 'Division by zero');
  return a / b;
}

export function testAssertMethods() {
  assert.ok(true);
  assert.equal(1, 1);
  assert.strictEqual(2, 2);
  assert.deepEqual({a: 1}, {a: 1});
}

export function locallyDefinedAssert() {
  const assert = () => {};
  assert();
}

export function capturedAssert() {
  const assert = () => {};
  function innerFunc() {
    assert();
  }
}

export function overrideInParam(assert) {
  assert("");
}

export function overrideInParamObjectPatPropAssign({ assert }) {
  assert("");
}

export function overrideInParamObjectPatPropKeyValue({ a: assert }) {
  assert("");
}

export function overrideInParamObjectPatPropKeyValueNested({ a: { assert } }) {
  assert("");
}

export function overrideInParamArray([assert]) {
  assert("");
}

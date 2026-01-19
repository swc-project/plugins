import assert from 'assert';

assert(true);

export function divide(a, b) {
  assert(b !== 0, 'Division by zero');
  return a / b;
}

export function testAssertMethods() {
  assert.ok(true);
  assert.equal(1, 1);
}

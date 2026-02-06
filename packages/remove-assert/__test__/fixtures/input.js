import assert from 'assert';

function divide(a, b) {
  assert(b !== 0, 'Division by zero');
  assert.strictEqual(typeof a, 'number', 'a must be a number');
  assert.deepEqual({ x: 1 }, { x: 1 });
  return a / b;
}

function process(data) {
  assert(data);
  assert.notEqual(data, null);
  return data.value;
}

// Direct assert calls
assert(true);
assert(1 === 1, 'one equals one');

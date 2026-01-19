assert(true, "assertion at top level");

export function shouldRemove() {
  assert(x > 0, "x must be positive");
  const result = compute();
  assert(result !== null, "result cannot be null");
  return result;
}

export function locallyDefinedAssert() {
  const assert = () => {};
  assert(true);
}

export function capturedAssert() {
  const assert = () => {};
  function innerFunc() {
    assert(true);
  }
}

export function overrideInParam(assert) {
  assert(true);
}

export function complexLogic() {
  if (x > 0) {
    assert(x < 100, "x out of range");
    process(x);
  }
  const arr = [1, 2, 3];
  assert(arr.length > 0, "array is empty");
  return arr;
}

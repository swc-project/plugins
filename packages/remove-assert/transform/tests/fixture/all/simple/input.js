assert(true, "assertion at top level");

export function shouldRemove() {
  assert(x > 0, "x must be positive");
  const result = compute();
  assert(result !== null, "result cannot be null");
  return result;
}

export function locallyDefinedAssert() {
  let assert = () => {};
  assert(true);
}

export function capturedAssert() {
  let assert = () => {};
  function innerFunc() {
    assert(true);
  }
}

export function overrideInParam(assert) {
  assert(true);
}

export function overrideInParamObjectPatPropAssign({ assert }) {
  assert(true);
}

export function overrideInParamObjectPatPropKeyValue({ c: assert }) {
  assert(true);
}

export function overrideInParamObjectPatPropKeyValueNested({ c: { assert } }) {
  assert(true);
}

export function overrideInParamArray([assert]) {
  assert(true);
}

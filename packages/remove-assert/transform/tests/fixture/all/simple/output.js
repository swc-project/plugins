;
export function shouldRemove() {
    ;
    const result = compute();
    ;
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

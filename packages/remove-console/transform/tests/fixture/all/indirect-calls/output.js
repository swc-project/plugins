;
;
export function log() {
    return void 0;
}
const result = void 0;
function local(console1) {
    return console1.log.apply(console1, args);
}
console.log.custom.call(console, sideEffect());
console[method].apply(console, args);
console.log["call"](console, sideEffect());

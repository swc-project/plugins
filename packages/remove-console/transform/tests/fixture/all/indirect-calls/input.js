console.log.call(console, sideEffect());
console.error.apply(undefined, sideEffect());
export function log() {
  return console.warn.call(console, sideEffect());
}
const result = console.log(sideEffect());
function local(console) {
  return console.log.apply(console, args);
}
console.log.custom.call(console, sideEffect());
console[method].apply(console, args);
console.log["call"](console, sideEffect());

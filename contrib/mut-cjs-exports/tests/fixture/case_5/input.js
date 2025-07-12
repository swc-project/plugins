export let a = function () {};

export function b() {}

export class c {}

a();
b();
new c();

let _ = {
  a,
  b,
  c,
};

a = function () {};
b = function () {};

a``;
b``;

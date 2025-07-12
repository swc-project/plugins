export function foo() {
  foo = () => 1;
  foo.bar = () => 2;
  return 3;
}

export let bar = function () {
  bar = () => 1;
  bar.bar = () => bar();
  return 3;
};

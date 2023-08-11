# `@swc/constify`

This plugin can be used to hoist constant parts of any expressions as constant variables, without affecting the runtime behavior or readability of the code.

## Why?

There are lots of cases where some parts of expressions are constant and thus can be reused, but others are not.

## Configuration

This plugin can be configured with `.swcrc`

```json
{
  "jsc": {
   "experimental": {
     "plugins": [ ["@swc/plugin-constify", {
     }] ]
   }
}
```

## Usage

Basically, this plugins is about extracting some parts of expressions.

```ts
import { constify, lazyConst } from "@swc/constify";

export function call(dynamic) {
  const options = [
    constify({
      code: 1,
      onClick() {},
    }),
    {
      code: 2,
      onClick() {
        console.log(dynamic);
      },
    },
    lazyConst({
      code: 3,
      onClick() {},
    }),
  ];

  return options;
}
```

becomes

```ts
const __CONST_0__ = {
  code: 1,
  onClick() {},
};
function __CONST_1__() {
  return (__CONST_1__ = function () {
    return {
      code: 3,
      onClick() {},
    };
  })();
}

export function call(dynamic) {
  const options = [
    __CONST_0__,
    {
      code: 2,
      onClick() {
        console.log(dynamic);
      },
    },
    __CONST_1__(),
  ];

  return options;
}
```

Note that this plugin supports using non-top-level variables. Those are hoisted to uppermost scope with the all used variable in the scope.
In other words, you can write code like

```ts
import { constify, lazyConst } from "@swc/constify";

export function call(d1) {
  function a(d2) {
    function a1() {
      return [
        constify({
          code: d1,
          onClick() {},
        }),
        constify({
          code: d2,
          onClick() {
            console.log(d2);
          },
        }),
        lazyConst({
          code: d1,
          onClick() {},
        }),
      ];
    }

    return a1;
  }

  return a;
}
```

and it will be compiled as

```ts
export function call(d1) {
  const __CONST_0__ = {
    code: d1,
    onClick() {},
  };

  function __CONST_2__() {
    return (__CONST_2__ = function () {
      return {
        code: d1,
        onClick() {},
      };
    })();
  }

  function a(d2) {
    const __CONST_1__ = {
      code: d2,
      onClick() {
        console.log(d2);
      },
    };

    function a1() {
      return [__CONST_0__, __CONST_1__, __CONST_2__()];
    }

    return a1;
  }

  return a;
}
```

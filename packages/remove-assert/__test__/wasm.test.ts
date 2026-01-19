import { expect, test } from "vitest";
import { transform } from "@swc/core";
import path from "node:path";
import url from "node:url";
import fs from "node:fs/promises";

const pluginName = "swc_plugin_remove_assert.wasm";

const transformCode = async (code: string, options = {}) => {
  return transform(code, {
    jsc: {
      parser: {
        syntax: "ecmascript",
      },
      target: "es2018",
      experimental: {
        plugins: [
          [
            path.join(
              path.dirname(url.fileURLToPath(import.meta.url)),
              "..",
              pluginName,
            ),
            options,
          ],
        ],
      },
    },
    filename: "test.js",
  });
};

test("Should load remove-assert wasm plugin correctly", async () => {
  const input = await fs.readFile(
    new URL("./fixtures/input.js", import.meta.url),
    "utf-8",
  );

  const { code } = await transformCode(input);
  expect(code).toMatchSnapshot();
});

test("Should handle import * as assert from 'assert'", async () => {
  const input = `import * as assert from 'assert';

assert(true, "top level assertion");

export function shouldRemove() {
  assert(x > 0, "x must be positive");
  const result = compute();
  assert(result !== null, "result cannot be null");
  return result;
}`;

  const { code } = await transformCode(input);
  expect(code).toMatchSnapshot();
});

test("Should handle import from 'node:assert'", async () => {
  const input = `import assert from 'node:assert';

assert(true, "top level assertion");

export function shouldRemove() {
  assert(x > 0, "x must be positive");
  const result = compute();
  assert(result !== null, "result cannot be null");
  return result;
}`;

  const { code } = await transformCode(input);
  expect(code).toMatchSnapshot();
});

test("Should handle import { assert, fail } from 'assert'", async () => {
  const input = `import { assert, fail } from 'assert';

assert(true, "top level assertion");
fail("should not reach here");

export function shouldRemove() {
  assert(x > 0, "x must be positive");
  const result = compute();
  assert(result !== null, "result cannot be null");
  fail("unreachable");
  return result;
}`;

  const { code } = await transformCode(input);
  expect(code).toMatchSnapshot();
});

test("Should handle separate import { assert } and import { fail }", async () => {
  const input = `import { assert } from 'assert';
import { fail } from 'assert';

assert(true, "top level assertion");
fail("should not reach here");

export function shouldRemove() {
  assert(x > 0, "x must be positive");
  const result = compute();
  assert(result !== null, "result cannot be null");
  fail("unreachable");
  return result;
}`;

  const { code } = await transformCode(input);
  expect(code).toMatchSnapshot();
});

test("Should handle import { fail } and import * as assert", async () => {
  const input = `import { fail } from 'assert';
import * as assert from 'assert';

assert(true, "top level assertion");
fail("should not reach here");

export function shouldRemove() {
  assert(x > 0, "x must be positive");
  const result = compute();
  assert(result !== null, "result cannot be null");
  fail("unreachable");
  return result;
}`;

  const { code } = await transformCode(input);
  expect(code).toMatchSnapshot();
});

test("Should handle assert method calls like assert.ok(), assert.strictEqual()", async () => {
  const input = `import assert from 'assert';

assert(true, "direct call");
assert.ok(value, "ok call");
assert.strictEqual(a, b, "strictEqual call");
assert.deepEqual(obj1, obj2, "deepEqual call");

export function shouldRemove() {
  assert(x > 0, "direct call in function");
  assert.ok(x, "ok call in function");
  const result = compute();
  assert.strictEqual(result, expected, "strictEqual call in function");
  return result;
}`;

  const { code } = await transformCode(input);
  expect(code).toMatchSnapshot();
});

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

test("Should remove all assert calls by default", async () => {
  const input = `
import assert from 'assert';
assert(true);
assert(1 === 1, 'message');
  `.trim();

  const { code } = await transformCode(input);
  expect(code).not.toContain("assert(");
});

test("Should remove assert method calls", async () => {
  const input = `
import assert from 'assert';
assert.strictEqual(1, 1);
assert.deepEqual({}, {});
  `.trim();

  const { code } = await transformCode(input);
  expect(code).not.toContain("assert.");
});

test("Should respect exclude option", async () => {
  const input = `
import assert from 'assert';
assert.strictEqual(1, 1);
assert.deepEqual({}, {});
  `.trim();

  const { code } = await transformCode(input, { exclude: ["strictEqual"] });
  expect(code).toContain("assert.strictEqual");
  expect(code).not.toContain("assert.deepEqual");
});

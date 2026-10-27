import { expect, test } from "vitest";
import { transform, type Options } from "@swc/core";
import path from "node:path";
import url from "node:url";

const options: Options = {
  jsc: {
    parser: { syntax: "ecmascript" },
    target: "esnext",
    experimental: {
      plugins: [
        [
          path.join(
            path.dirname(url.fileURLToPath(import.meta.url)),
            "..",
            "swc_plugin_import_meta_env.wasm",
          ),
          {},
        ],
      ],
    },
  },
};

test.each([
  ["import.meta.env;", "process.env;"],
  ["import.meta.env.MODE;", "process.env.MODE;"],
  ['import.meta.env["PROP"];', 'process.env["PROP"];'],
  ["import.meta.env?.MODE;", "process.env?.MODE;"],
  [
    "function read() { return useEnv(import.meta.env, () => import.meta.env.MODE); }",
    "function read() { return useEnv(process.env, () => process.env.MODE); }",
  ],
  [
    "import.meta.env.MODE = import.meta.env.OTHER;",
    "process.env.MODE = process.env.OTHER;",
  ],
  [
    'const values = [import.meta, import.meta.url, import.meta.glob("./*.js"), meta.env, process.env, import.meta.environment, import.meta["env"]];',
    'const values = [import.meta, import.meta.url, import.meta.glob("./*.js"), meta.env, process.env, import.meta.environment, import.meta["env"]];',
  ],
])("transforms %s", async (input, expected) => {
  const actual = await transform(input, options);
  const output = await transform(expected, {
    jsc: { parser: { syntax: "ecmascript" }, target: "esnext" },
  });
  expect(actual.code).toBe(output.code);
});

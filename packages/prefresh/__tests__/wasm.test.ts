import { expect, test } from "vitest";
import { transform } from "@swc/core";
import path from "node:path";
import url from "node:url";

const pluginName = "swc_plugin_prefresh.wasm";

const transformCode = async (
  code: string,
  options = {
    library: ["@custom/preact", "preact", "react"],
  },
) => {
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
    filename: "test.ts",
  });
};

test("Should load prefresh wasm plugin correctly", async () => {
  const input = `import { createContext } from 'preact';

export function aaa() {
  const context = createContext();
}`;

  const { code } = await transformCode(input);

  expect(code).match(/const context = createContext\[`.+\$context1`\]/);
});

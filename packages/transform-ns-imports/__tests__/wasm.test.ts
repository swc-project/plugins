import { expect, test, describe } from "vitest";
import { transform } from "@swc/core";
import path from "node:path";
import url from "node:url";
import fs from "node:fs/promises";

const pluginName = "swc_plugin_transform_ns_imports.wasm";

const transformCode = async (
  code: string,
  options: Record<string, unknown> = {
    "rewrite-module-namespace": {
      rewrite: "rewrite-module-namespace/swc-proxy",
    },
  },
) => {
  return transform(code, {
    jsc: {
      parser: {
        syntax: "ecmascript",
        jsx: true,
      },
      target: "es3",
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

async function walkDir(
  dir: URL,
  callback: (
    dir: string,
    input: string,
    config?: Record<string, unknown>,
  ) => Promise<void>,
) {
  const dirs = await fs.readdir(dir);
  const baseDir = url.fileURLToPath(dir);

  for (const dir of dirs) {
    const inputFilePath = path.join(baseDir, dir, "input.js");
    const configPath = path.join(baseDir, dir, "config.json");

    const config = await fs.readFile(configPath, "utf-8").then(
      (json) => {
        return JSON.parse(json);
      },
      (_) => undefined,
    );

    try {
      const input = await fs.readFile(inputFilePath, "utf-8");
      await callback(dir, input, config);
    } catch (e) {
      console.log(e);
    }
  }
}

describe("Should load transform-ns-imports wasm plugin correctly", async () => {
  await walkDir(
    new URL("../transform/tests/fixture", import.meta.url),
    async (dir, input, config) => {
      await test(`Should transform ${dir} correctly`, async () => {
        const { code } = await transformCode(input, config);
        expect(code).toMatchSnapshot();
      });
    },
  );
});

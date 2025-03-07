import { describe, expect, test } from "vitest";
import path from "node:path";
import fs from "node:fs/promises";
import url from "node:url";
import { type Options, transform } from "@swc/core";

const pluginName = "swc_plugin_emotion.wasm";

const options: Options = {
  jsc: {
    parser: {
      syntax: "ecmascript",
    },
    experimental: {
      plugins: [
        [
          path.join(
            path.dirname(url.fileURLToPath(import.meta.url)),
            "..",
            pluginName,
          ),
          {},
        ],
      ],
    },
  },
};

test("Should transform emotion css correctly", async () => {
  const code = await fs.readFile(
    path.resolve(
      url.fileURLToPath(import.meta.url),
      "..",
      "fixtures",
      "input.js",
    ),
    "utf-8",
  );
  const output = await transform(code, options);
  expect(output.code).toMatchSnapshot();
});

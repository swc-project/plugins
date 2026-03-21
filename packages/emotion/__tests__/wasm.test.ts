import { describe, expect, test } from "vitest";
import path from "node:path";
import fs from "node:fs/promises";
import url from "node:url";
import { type Options, transform } from "@swc/core";

const pluginName = "swc_plugin_emotion.wasm";
const pluginPath = path.join(
  path.dirname(url.fileURLToPath(import.meta.url)),
  "..",
  pluginName,
);

const options: Options = {
  jsc: {
    parser: {
      syntax: "ecmascript",
    },
    experimental: {
      plugins: [
        [
          pluginPath,
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

test("Should add label to css tagged template when autoLabel is 'always'", async () => {
  const code = await fs.readFile(
    path.resolve(
      url.fileURLToPath(import.meta.url),
      "..",
      "fixtures",
      "auto-label-input.js",
    ),
    "utf-8",
  );
  const output = await transform(code, {
    jsc: {
      parser: {
        syntax: "ecmascript",
      },
      experimental: {
        plugins: [
          [
            pluginPath,
            {
              autoLabel: "always",
              sourceMap: false,
            },
          ],
        ],
      },
    },
  });
  expect(output.code).toMatchSnapshot();
  // Verify labels are added with the correct "label:" prefix
  expect(output.code).toContain('"label:testCls"');
  expect(output.code).toContain('"label:anotherStyle"');
});

test("Should not add label to css tagged template when autoLabel is 'never'", async () => {
  const code = await fs.readFile(
    path.resolve(
      url.fileURLToPath(import.meta.url),
      "..",
      "fixtures",
      "auto-label-input.js",
    ),
    "utf-8",
  );
  const output = await transform(code, {
    jsc: {
      parser: {
        syntax: "ecmascript",
      },
      experimental: {
        plugins: [
          [
            pluginPath,
            {
              autoLabel: "never",
              sourceMap: false,
            },
          ],
        ],
      },
    },
  });
  expect(output.code).not.toContain('"label:testCls"');
  expect(output.code).not.toContain('"label:anotherStyle"');
});

import { expect, test } from "vitest";
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
const fixtureDir = path.resolve(
  url.fileURLToPath(import.meta.url),
  "..",
  "fixtures",
);

async function readFixture(name: string) {
  return fs.readFile(path.resolve(fixtureDir, name), "utf-8");
}

async function transformWithEmotion(
  code: string,
  pluginOptions: Record<string, unknown> = {},
  envName?: string,
) {
  return transform(code, {
    envName,
    jsc: {
      parser: {
        syntax: "ecmascript",
      },
      experimental: {
        plugins: [[pluginPath, pluginOptions]],
      },
    },
  });
}

const options: Options = {
  jsc: {
    parser: {
      syntax: "ecmascript",
    },
    experimental: {
      plugins: [[pluginPath, {}]],
    },
  },
};

test("Should transform emotion css correctly", async () => {
  const code = await readFixture("input.js");
  const output = await transform(code, options);
  expect(output.code).toMatchSnapshot();
});

test("Should add label to css tagged template when autoLabel is 'always'", async () => {
  const code = await readFixture("auto-label-input.js");
  const output = await transformWithEmotion(code, {
    autoLabel: "always",
    sourceMap: false,
  });
  expect(output.code).toMatchSnapshot();
  // Verify labels are added with the correct "label:" prefix
  expect(output.code).toContain('"label:testCls"');
  expect(output.code).toContain('"label:anotherStyle"');
});

test("Should not add label to css tagged template when autoLabel is 'never'", async () => {
  const code = await readFixture("auto-label-input.js");
  const output = await transformWithEmotion(code, {
    autoLabel: "never",
    sourceMap: false,
  });
  expect(output.code).not.toContain('"label:testCls"');
  expect(output.code).not.toContain('"label:anotherStyle"');
});

test("Should keep plain keyframes label when sourceMap is enabled", async () => {
  const code = await readFixture("keyframes-input.js");
  const output = await transformWithEmotion(
    code,
    {
      autoLabel: "always",
      sourceMap: true,
    },
    "development",
  );

  expect(output.code).toContain(
    'keyframes("0%{opacity:1;}50%{opacity:0.5;}100%{opacity:1;}", "pulse", "/*# sourceMappingURL=',
  );
  expect(output.code).not.toContain("label:pulse");
});

test("Should keep plain keyframes label when sourceMap is disabled", async () => {
  const code = await readFixture("keyframes-input.js");
  const output = await transformWithEmotion(code, {
    autoLabel: "always",
    sourceMap: false,
  });

  expect(output.code).toContain(
    'keyframes("0%{opacity:1;}50%{opacity:0.5;}100%{opacity:1;}", "pulse");',
  );
  expect(output.code).not.toContain("label:pulse");
});

test("Should keep plain keyframes label for namespace imports", async () => {
  const code = await readFixture("keyframes-namespace-input.js");
  const output = await transformWithEmotion(
    code,
    {
      autoLabel: "always",
      sourceMap: true,
    },
    "development",
  );

  expect(output.code).toContain(
    'emotionReact.keyframes("0%{opacity:1;}50%{opacity:0.5;}100%{opacity:1;}", "pulse", "/*# sourceMappingURL=',
  );
  expect(output.code).not.toContain("label:pulse");
});

test("Should transform css prop output from React Compiler", async () => {
  const output = await transform(
    `
      import { css } from "@emotion/react";

      export function App() {
        return <div css={css\`width:120px;\`} />;
      }
    `,
    {
      envName: "development",
      jsc: {
        parser: {
          syntax: "ecmascript",
          jsx: true,
        },
        transform: {
          react: {
            runtime: "automatic",
            importSource: "@emotion/react",
          },
          reactCompiler: true,
        },
        experimental: {
          plugins: [[pluginPath, {}]],
        },
      },
    } satisfies Options,
  );

  expect(output.code).toContain('css("width:120px;", "")');
});

test("Should not wrap dynamic css prop arrays", async () => {
  const output = await transform(
    `
      import { css } from "@emotion/react";
      import { forwardRef } from "react";

      const styles = {
        row: (theme) => css({ display: "grid", gap: theme.spacing.sm }),
      };

      export const Row = forwardRef((props, ref) => (
        <div ref={ref} css={[styles.row, {}]} {...props} />
      ));
    `,
    {
      jsc: {
        parser: {
          syntax: "ecmascript",
          jsx: true,
        },
        transform: {
          react: {
            runtime: "automatic",
            importSource: "@emotion/react",
          },
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
    } satisfies Options,
  );

  expect(output.code).toContain("css: [");
  expect(output.code).toContain("styles.row");
  expect(output.code).not.toContain("css([");
});

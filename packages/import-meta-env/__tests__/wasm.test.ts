import { expect, test } from "vitest";
import { transform } from "@swc/core";
import path from "node:path";
import url from "node:url";

const pluginName = "swc_plugin_import_meta_env.wasm";

const transformCode = async (
  code: string,
  parser:
    | { syntax: "ecmascript"; jsx?: boolean }
    | { syntax: "typescript"; tsx?: boolean } = {
    syntax: "ecmascript",
  },
) => {
  return transform(code, {
    jsc: {
      parser,
      target: "es2018",
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
    filename:
      parser.syntax === "typescript" && parser.tsx ? "test.tsx" : "test.js",
  });
};

test("transforms import.meta.env expressions to process.env", async () => {
  const { code } = await transformCode(`
const env = import.meta.env;
const mode = import.meta.env.MODE;
const prop = import.meta.env["PROP"];
`);

  expect(code).toContain("const env = process.env;");
  expect(code).toContain("const mode = process.env.MODE;");
  expect(code).toContain('const prop = process.env["PROP"];');
  expect(code).not.toContain("import.meta.env");
});

test("leaves other import.meta expressions unchanged", async () => {
  const { code } = await transformCode(`
const meta = import.meta;
const glob = import.meta.glob("./*.ts");
const computed = import.meta["env"];
`);

  expect(code).toContain("const meta = import.meta;");
  expect(code).toContain('const glob = import.meta.glob("./*.ts");');
  expect(code).toContain('const computed = import.meta["env"];');
  expect(code).not.toContain("process.env");
});

test("works with TypeScript and TSX input", async () => {
  const { code } = await transformCode(
    `
const Component = () => <span>{import.meta.env.MODE as string}</span>;
`,
    {
      syntax: "typescript",
      tsx: true,
    },
  );

  expect(code).toContain("process.env.MODE");
  expect(code).not.toContain("import.meta.env");
});

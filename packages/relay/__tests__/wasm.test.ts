import { expect, test } from "vitest";
import { transform } from "@swc/core";
import path from "node:path";
import url from "node:url";

const pluginName = "swc_plugin_relay.wasm";

const transformCode = async (
  code: string,
  options = {
    rootDir: "",
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
    filename: "test.js",
  });
};

test("Should load relay wasm plugin correctly", async () => {
  const input = `const myFragment = graphql\`
  fragment FooFragment on Bar {
    id
  }
\`;
useQuery(graphql\`
  query FooQuery {
    id
  }
\`);
`;
  const { code } = await transformCode(input);
  expect(code).toMatchSnapshot();
});

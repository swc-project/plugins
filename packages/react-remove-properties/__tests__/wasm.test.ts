import { expect, test } from "vitest";
import { transform } from "@swc/core";
import path from "node:path";
import url from "node:url";

const pluginName = "swc_plugin_react_remove_properties.wasm";

const transformCode = async (code: string, options = {}) => {
  return transform(code, {
    jsc: {
      parser: {
        syntax: "ecmascript",
        jsx: true,
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
    filename: "test.jsx",
  });
};

test("Should load react-remove-properties wasm plugin correctly", async () => {
  const input = `export default function Home() {
  return (
    <div data-test-id="1" data-custom="1a">
      <div data-custom="2">
        <h1 data-testid="3">Hello World!</h1>
      </div>
    </div>
  );
}
`;
  const { code } = await transformCode(input);
  expect(code).toMatchSnapshot();
});

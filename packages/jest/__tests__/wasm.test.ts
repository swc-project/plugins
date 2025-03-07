import { describe, it, expect } from "vitest";
import { transform, type Options } from "@swc/core";
import path from "node:path";
import url from "node:url";

const options: Options = {
  jsc: {
    parser: {
      syntax: "ecmascript",
      jsx: true,
    },
    experimental: {
      plugins: [
        [
          path.join(
            path.dirname(url.fileURLToPath(import.meta.url)),
            "..",
            "swc_plugin_jest.wasm",
          ),
          {},
        ],
      ],
    },
  },
};

describe("jest swc plugin", () => {
  it("should hoist jest.mock calls", async () => {
    const input = `
      console.log('before mock');
      jest.mock('./some-module');
      console.log('after mock');
    `;

    const output = await transform(input, options);

    // Remove whitespace and newlines for comparison
    const normalizedOutput = output.code.replace(/\s+/g, "");
    const expectedOutput = `
      jest.mock('./some-module');
      console.log('before mock');
      console.log('after mock');
    `.replace(/\s+/g, "");

    expect(normalizedOutput).toBe(expectedOutput);
  });

  it("should hoist multiple jest method calls", async () => {
    const input = `
      console.log('start');
      jest.unmock('./module-a');
      var something = true;
      jest.mock('./module-b');
      jest.enableAutomock();
      console.log('end');
    `;

    const output = await transform(input, options);

    // Remove whitespace and newlines for comparison
    const normalizedOutput = output.code.replace(/\s+/g, "");
    const expectedOutput = `
      jest.unmock('./module-a');
      jest.mock('./module-b');
      jest.enableAutomock();
      console.log('start');
      var something = true;
      console.log('end');
    `.replace(/\s+/g, "");

    expect(normalizedOutput).toBe(expectedOutput);
  });

  it("should not hoist non-hoistable jest methods", async () => {
    const input = `
      console.log('start');
      jest.spyOn(something, 'method');
      jest.mock('./module');
      console.log('end');
    `;

    const output = await transform(input, options);

    // Remove whitespace and newlines for comparison
    const normalizedOutput = output.code.replace(/\s+/g, "");
    const expectedOutput = `
      jest.mock('./module');
      console.log('start');
      jest.spyOn(something, 'method');
      console.log('end');
    `.replace(/\s+/g, "");

    expect(normalizedOutput).toBe(expectedOutput);
  });
});

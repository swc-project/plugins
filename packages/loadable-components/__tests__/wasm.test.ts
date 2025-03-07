import { test, expect, describe } from "vitest";
import { transform } from "@swc/core";
import path from "node:path";
import url from "node:url";

describe("swc-plugin-loadable-components", () => {
  const pluginName = "swc_plugin_loadable_components.wasm";

  const transformCode = async (code: string, options = {}) => {
    return transform(code, {
      jsc: {
        parser: {
          syntax: "typescript",
          tsx: true,
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
      filename: "test.tsx",
    });
  };

  test("should transform basic loadable import", async () => {
    const input = `
      import loadable from '@loadable/component';
      const MyComponent = loadable(() => import('./Component'));
    `;

    const { code } = await transformCode(input);

    expect(code).toContain("resolved: {}");
    expect(code).toContain("chunkName ()");
    expect(code).toContain("isReady (props)");
    expect(code).toContain("importAsync:");
    expect(code).toContain("requireAsync (props)");
    expect(code).toContain("requireSync (props)");
    expect(code).toContain("resolve ()");
  });

  test("should handle custom chunk names", async () => {
    const input = `
      import loadable from '@loadable/component';
      const MyComponent = loadable(() => /* webpackChunkName: "my-chunk" */ import('./Component'));
    `;

    const { code } = await transformCode(input);

    expect(code).toContain('"my-chunk"');
  });

  test("should handle dynamic imports with template literals", async () => {
    const input = `
      import loadable from '@loadable/component';
      const MyComponent = loadable((props) => import(\`./components/\${props.name}\`));
    `;

    const { code } = await transformCode(input);

    expect(code).toContain('"components-');
    expect(code).toContain(".replace(");
  });

  test("should handle custom loadable import names", async () => {
    const input = `
      import { lazy } from '@loadable/component';
      const MyComponent = lazy(() => import('./Component'));
    `;

    const { code } = await transformCode(input);

    expect(code).toContain("resolved: {}");
    expect(code).toContain("chunkName ()");
  });

  test("should handle loadable comment marker", async () => {
    const input = `
      import loadable from '@loadable/component';
      const MyComponent = {
        loader: /* #__LOADABLE__ */ () => import('./Component')
      };
    `;

    const { code } = await transformCode(input);

    expect(code).toContain("resolved: {}");
    expect(code).toContain("chunkName ()");
  });

  test("should handle custom signatures", async () => {
    const input = `
      import { lazyLoad } from 'my-loadable';
      const MyComponent = lazyLoad(() => import('./Component'));
    `;

    const { code } = await transformCode(input, {
      signatures: [
        {
          from: "my-loadable",
          name: "lazyLoad",
        },
      ],
    });

    expect(code).toContain("resolved: {}");
    expect(code).toContain("chunkName ()");
  });
});

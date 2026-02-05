import { expect, test, describe } from "vitest";
import { transform } from "@swc/core";
import path from "node:path";
import url from "node:url";

const pluginName = "swc_plugin_experimental_feature_flags.wasm";

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
    filename: "test.js",
  });
};

describe("Mark mode (default) - marker generation", () => {
  test("Should mark flags with __SWC_FLAGS__ markers", async () => {
    const input = `import { useExperimentalFlags } from '@their/library';

function App() {
  const { featureA, featureB } = useExperimentalFlags();

  if (featureA) {
    console.log('Feature A is enabled');
  }

  return featureB ? 'Beta' : 'Stable';
}`;

    const { code } = await transformCode(input, {
      mode: "mark",
      libraries: {
        "@their/library": {
          functions: ["useExperimentalFlags"],
        },
      },
    });

    expect(code).toMatchSnapshot();
    // Should have eliminated the import
    expect(code).not.toContain("import");
    expect(code).not.toContain("useExperimentalFlags");
    // Should have markers
    expect(code).toContain("__SWC_FLAGS__.featureA");
    expect(code).toContain("__SWC_FLAGS__.featureB");
    // Should NOT have performed DCE
    expect(code).toContain("? 'Beta' : 'Stable'");
  });

  test("Should support custom marker object", async () => {
    const input = `import { useExperimentalFlags } from '@their/library';

function App() {
  const { featureA } = useExperimentalFlags();
  return featureA ? 'On' : 'Off';
}`;

    const { code } = await transformCode(input, {
      mode: "mark",
      libraries: {
        "@their/library": {
          functions: ["useExperimentalFlags"],
        },
      },
      markerObject: "__CUSTOM_FLAGS__",
    });

    expect(code).toMatchSnapshot();
    expect(code).toContain("__CUSTOM_FLAGS__.featureA");
    expect(code).not.toContain("__SWC_FLAGS__");
  });
});

describe("Shake mode - DCE on markers", () => {
  test("Should substitute markers and eliminate dead code", async () => {
    const input = `function App() {
  if (__SWC_FLAGS__.featureA) {
    console.log('Feature A is enabled');
  }

  return __SWC_FLAGS__.featureB ? 'Beta' : 'Stable';
}`;

    const { code } = await transformCode(input, {
      mode: "shake",
      flagValues: {
        featureA: true,
        featureB: false,
      },
    });

    expect(code).toMatchSnapshot();
    // Should have eliminated markers
    expect(code).not.toContain("__SWC_FLAGS__");
    // Should have optimized the if statement
    expect(code).toContain("console.log('Feature A is enabled')");
    // Should have optimized the ternary
    expect(code).toContain("'Stable'");
    expect(code).not.toContain("'Beta'");
  });

  test("Should handle complex logical operations", async () => {
    const input = `function App() {
  // Logical AND with true
  const useNew = __SWC_FLAGS__.enableFeature && hasPermission();

  // Logical OR with false
  const useLegacy = __SWC_FLAGS__.showBeta || !isModern();

  if (__SWC_FLAGS__.enableFeature && __SWC_FLAGS__.showBeta) {
    return 'both';
  } else if (__SWC_FLAGS__.enableFeature) {
    return 'feature-only';
  } else {
    return 'none';
  }
}`;

    const { code } = await transformCode(input, {
      mode: "shake",
      flagValues: {
        enableFeature: true,
        showBeta: false,
      },
    });

    expect(code).toMatchSnapshot();
    expect(code).not.toContain("__SWC_FLAGS__");
    expect(code).toContain("hasPermission()");
    expect(code).toContain("!isModern()");
    expect(code).toContain("'feature-only'");
  });
});

describe("Configuration defaults", () => {
  test("Should default to mark mode when mode is not specified", async () => {
    const input = `import { useExperimentalFlags } from '@their/library';

function App() {
  const { featureA } = useExperimentalFlags();
  return featureA ? 'On' : 'Off';
}`;

    // When mode is not specified, it defaults to "mark"
    const { code } = await transformCode(input, {
      libraries: {
        "@their/library": {
          functions: ["useExperimentalFlags"],
        },
      },
    });

    expect(code).toMatchSnapshot();
    // Should have created markers (mark mode is default)
    expect(code).toContain("__SWC_FLAGS__.featureA");
  });
});

describe("Multiple libraries", () => {
  test("Should handle multiple library sources in mark mode", async () => {
    const input = `import { useExperimentalFlags } from '@their/library';
import { getFlags } from '@another/flags';

function App() {
  const { featureA } = useExperimentalFlags();
  const { featureB } = getFlags();

  return featureA && featureB ? 'Both' : 'Neither';
}`;

    const { code } = await transformCode(input, {
      mode: "mark",
      libraries: {
        "@their/library": {
          functions: ["useExperimentalFlags"],
        },
        "@another/flags": {
          functions: ["getFlags"],
        },
      },
    });

    expect(code).toMatchSnapshot();
    expect(code).not.toContain("import");
    expect(code).not.toContain("useExperimentalFlags");
    expect(code).not.toContain("getFlags");
    expect(code).toContain("__SWC_FLAGS__.featureA");
    expect(code).toContain("__SWC_FLAGS__.featureB");
  });
});

describe("Edge cases", () => {
  test("Should handle nested scopes correctly in mark mode", async () => {
    const input = `import { useExperimentalFlags } from '@their/library';

function App() {
  const { featureA } = useExperimentalFlags();

  if (featureA) {
    const featureA = false; // Shadowed variable
    if (featureA) {
      console.log('This uses the local featureA');
    }
  }
}`;

    const { code } = await transformCode(input, {
      mode: "mark",
      libraries: {
        "@their/library": {
          functions: ["useExperimentalFlags"],
        },
      },
    });

    expect(code).toMatchSnapshot();
    // Outer featureA should be marked
    expect(code).toContain("__SWC_FLAGS__.featureA");
    // Inner shadowed variable should remain
    expect(code).toContain("false");
  });
});

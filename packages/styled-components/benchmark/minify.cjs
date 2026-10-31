// Usage: node minify.cjs <baseline.wasm> <candidate.wasm> <fixture-directory>
// Both artifacts should use the same toolchain, lockfile and release flags.
const assert = require("node:assert/strict");
const { createHash } = require("node:crypto");
const { readdirSync, readFileSync } = require("node:fs");
const { join, resolve } = require("node:path");
const { performance } = require("node:perf_hooks");
const swc = require("@swc/core");

const [baselinePath, candidatePath, fixturePath] = process.argv.slice(2);
assert(
  baselinePath && candidatePath && fixturePath,
  "Usage: node minify.cjs <baseline.wasm> <candidate.wasm> <fixture-directory>",
);
function list(dir) {
  return readdirSync(dir, { withFileTypes: true })
    .flatMap((entry) =>
      entry.isDirectory()
        ? list(join(dir, entry.name))
        : entry.name.endsWith(".js")
          ? [join(dir, entry.name)]
          : [],
    )
    .sort();
}
const files = list(resolve(fixturePath)).map((filename) => ({
  filename,
  source: readFileSync(filename, "utf8"),
}));
assert(files.length, "No JavaScript fixtures found");
const artifacts = [baselinePath, candidatePath].map((path) => {
  path = resolve(path);
  const bytes = readFileSync(path);
  return {
    path,
    bytes: bytes.length,
    sha256: createHash("sha256").update(bytes).digest("hex"),
  };
});
function options(filename, plugin, minify) {
  return {
    filename,
    minify: false,
    sourceMaps: false,
    module: { type: "es6" },
    jsc: {
      parser: { syntax: "ecmascript", jsx: true },
      target: "es2022",
      transform: { react: { runtime: "preserve" } },
      experimental: {
        plugins: [
          [
            plugin,
            {
              cssProp: true,
              displayName: true,
              fileName: true,
              meaninglessFileNames: ["index"],
              minify,
              namespace: "",
              pure: true,
              ssr: true,
              topLevelImportPaths: [],
              transpileTemplateLiterals: true,
            },
          ],
        ],
      },
    },
  };
}
// Compare identical options, rather than comparing minified to unminified CSS.
for (const minify of [true, false]) {
  for (const file of files) {
    const outputs = artifacts.map(
      ({ path }) =>
        swc.transformSync(file.source, options(file.filename, path, minify))
          .code,
    );
    assert.equal(outputs[1], outputs[0], `${file.filename}, minify=${minify}`);
  }
}
const cases = [true, false].flatMap((minify) =>
  artifacts.map(({ path }, index) => ({
    name: `${index ? "candidate" : "baseline"}-minify-${minify}`,
    inputs: files.map((file) => ({
      ...file,
      options: options(file.filename, path, minify),
    })),
  })),
);
const result = {
  core: swc.version,
  node: process.version,
  artifacts,
  files: files.length,
  sourceBytes: files.reduce(
    (sum, file) => sum + Buffer.byteLength(file.source),
    0,
  ),
  parity: "Byte-identical JS for both minify settings",
  runs: [],
};
let checksum = 0;
for (let round = 0; round < 3; round++) {
  for (let k = 0; k < cases.length; k++) {
    const { name, inputs } = cases[(round + k) % cases.length];
    const run = () => {
      for (const file of inputs)
        checksum += swc.transformSync(file.source, file.options).code.length;
    };
    let end = performance.now() + 1000;
    while (performance.now() < end) run();
    const samples = [];
    end = performance.now() + 1000;
    while (performance.now() < end) {
      const start = performance.now();
      run();
      samples.push(performance.now() - start);
    }
    samples.sort((a, b) => a - b);
    const row = {
      round: round + 1,
      name,
      medianMs: samples[Math.floor(samples.length / 2)],
      samples: samples.length,
    };
    result.runs.push(row);
    console.error(JSON.stringify(row));
  }
}
result.checksum = checksum;
console.log(JSON.stringify(result, null, 2));

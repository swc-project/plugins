import {expect, test, describe} from "vitest";
import {transform} from "@swc/core";
import path from "node:path";
import url from "node:url";
import fs from "node:fs/promises";

const pluginName = "swc_plugin_transform_imports.wasm";

const transformCode = async (
    code: string,
    options: Record<string, unknown> = {
        "react-bootstrap": {
            transform: "react-bootstrap/lib/{{member}}",
            preventFullImport: false,
            skipDefaultConversion: false,
            handleDefaultImport: true,
            handleNamespaceImport: true,
        },
        "my-library/?(((\\w*)?/?)*)": {
            transform: "my-library/{{ matches.[1] }}/{{member}}",
            preventFullImport: false,
            skipDefaultConversion: false,
            handleDefaultImport: true,
            handleNamespaceImport: true,
        },

        "rewrite-module-namespace": {
            transform: "rewrite-module-namespace/{{ member }}",
            prevent_full_import: false,
            skip_default_conversion: true,
            handle_default_import: true,
            handle_namespace_import: true,
            rewrite_namespace_to_proxy: true
        },
        "my-library-2": {
            transform: "my-library-2/{{ camelCase member }}",
            preventFullImport: false,
            skipDefaultConversion: true,
            handleDefaultImport: true,
            handleNamespaceImport: true,
        },
        "my-library-4": {
            transform: [
                ["foo", "my-library-4/this_is_foo"],
                ["bar", "my-library-4/this_is_bar"],
                [
                    "use(\\w*)",
                    "my-library-4/{{ kebabCase member }}/{{ kebabCase memberMatches.[1] }}",
                ],
                ["(\\w*)Icon", "my-library-4/{{ kebabCase memberMatches.[1] }}"],
                ["*", "my-library-4/{{ kebabCase member }}"],
            ],
            preventFullImport: false,
            skipDefaultConversion: false,
            handleDefaultImport: true,
            handleNamespaceImport: true,
        },
        "my-library-3": {
            transform: "my-library-3/{{ kebabCase member }}",
            preventFullImport: false,
            skipDefaultConversion: true,
            handleDefaultImport: true,
            handleNamespaceImport: true,
        },
        "^(\\..*)(\\.tsx?)$": {
            transform: "{{matches.[1]}}.js",
            preventFullImport: false,
            skipDefaultConversion: false,
            handleDefaultImport: false,
            handleNamespaceImport: false,
        },
    },
) => {
    return transform(code, {
        jsc: {
            parser: {
                syntax: "ecmascript",
                jsx: true,
            },
            target: "es3",
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

async function walkDir(
    dir: URL,
    callback: (
        dir: string,
        input: string,
        config?: Record<string, unknown>,
    ) => Promise<void>,
) {
    const dirs = await fs.readdir(dir);
    const baseDir = url.fileURLToPath(dir);

    for (const dir of dirs) {
        const inputFilePath = path.join(baseDir, dir, "input.js");
        const configPath = path.join(baseDir, dir, "config.json");

        const config = await fs.readFile(configPath, "utf-8").then(
            (json) => {
                return JSON.parse(json);
            },
            (_) => undefined,
        );

        try {
            const input = await fs.readFile(inputFilePath, "utf-8");
            await callback(dir, input, config);
        } catch (e) {
            console.log(e);
        }
    }
}

describe("Should load transform-imports wasm plugin correctly", async () => {
    await walkDir(
        new URL("../transform/tests/fixture", import.meta.url),
        async (dir, input, config) => {
            await test(`Should transform ${dir} correctly`, async () => {
                const {code} = await transformCode(input, config);
                expect(code).toMatchSnapshot();
            });
        },
    );
});

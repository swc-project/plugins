import { describe, expect, it } from "vitest";
import { transform } from "@swc/core";
import path from "node:path";
import url from "node:url";

const transformCode = async (code: string, options = {}) => {
  const result = await transform(code, {
    jsc: {
      parser: {
        syntax: "typescript",
        tsx: true,
      },
      experimental: {
        plugins: [
          [
            path.join(
              path.dirname(url.fileURLToPath(import.meta.url)),
              "..",
              "swc_plugin_formatjs.wasm",
            ),
            options,
          ],
        ],
      },
    },
  });
  return result.code;
};

describe("formatjs swc plugin", () => {
  it("should transform FormattedMessage component", async () => {
    const input = `
      import React from 'react';
      import { FormattedMessage } from 'react-intl';

      export function Greeting() {
        return (
          <FormattedMessage
            defaultMessage="Hello, {name}!"
            description="Greeting message"
          />
        );
      }
    `;

    const output = await transformCode(input);

    expect(output).toMatch(/id: \".+\"/);
    expect(output).toMatch(/defaultMessage: "Hello, \{name\}!"/);
    expect(output).not.toMatch(/description/);
  });

  it("should transform defineMessage function", async () => {
    const input = `
      import { defineMessage } from 'react-intl';

      const message = defineMessage({
        defaultMessage: "Welcome to {site}",
        description: "Welcome message"
      });
    `;

    const output = await transformCode(input);

    expect(output).toMatch(/id: "[^"]+"/);
    expect(output).toMatch(/defaultMessage: "Welcome to \{site\}"/);
    expect(output).not.toMatch(/description/);
  });

  it("should transform multiple messages in defineMessages", async () => {
    const input = `
      import { defineMessages } from 'react-intl';

      const messages = defineMessages({
        greeting: {
          defaultMessage: "Hello",
          description: "Greeting"
        },
        farewell: {
          defaultMessage: "Goodbye",
          description: "Farewell"
        }
      });
    `;

    const output = await transformCode(input);

    const idMatches = output.match(/id:/g);
    expect(idMatches).toHaveLength(2);

    expect(output).toMatch(/defaultMessage: "Hello"/);
    expect(output).toMatch(/defaultMessage: "Goodbye"/);

    expect(output).not.toMatch(/description/);
  });

  it("should handle formatMessage calls", async () => {
    const input = `
      import { useIntl } from 'react-intl';

      function MyComponent() {
        const intl = useIntl();
        return intl.formatMessage({
          defaultMessage: "Click here",
          description: "Button text"
        });
      }
    `;

    const output = await transformCode(input);

    expect(output).toMatch(/id: "[^"]+"/);
    expect(output).toMatch(/defaultMessage: "Click here"/);
    expect(output).not.toMatch(/description/);
  });

  it("should preserve whitespace when option is enabled", async () => {
    const input = `
      import { FormattedMessage } from 'react-intl';

      export function Greeting() {
        return (
          <FormattedMessage
            defaultMessage="Hello,    {name}!"
            description="Greeting message"
          />
        );
      }
    `;

    const output = await transformCode(input, {
      preserve_whitespace: true,
    });

    expect(output).toMatch(/defaultMessage: "Hello, {4}\{name\}!"/);
  });

  it("should use custom id interpolation pattern", async () => {
    const input = `
      import { FormattedMessage } from 'react-intl';

      export function Greeting() {
        return (
          <FormattedMessage
            defaultMessage="Hello, {name}!"
            description="Greeting message"
          />
        );
      }
    `;

    const output = await transformCode(input, {
      idInterpolationPattern: "[name]_[hash:base64:5]",
    });

    expect(output).toMatch(/id: "file_[a-zA-Z0-9]{5}"/);
  });

  it("should handle additional component names", async () => {
    const input = `
      import { CustomMessage } from './custom-intl';

      export function Greeting() {
        return (
          <CustomMessage
            defaultMessage="Hello, {name}!"
            description="Greeting message"
          />
        );
      }
    `;

    const output = await transformCode(input, {
      additionalComponentNames: ["CustomMessage"],
    });

    expect(output).toMatch(/id: "[^"]+"/);
    expect(output).toMatch(/defaultMessage: "Hello, \{name\}!"/);
    expect(output).not.toMatch(/description/);
  });

  it("should be able to use sha1 and sha512 hashing in interpolation", async () => {
    const input = `
      import { FormattedMessage } from 'react-intl';

      export function Greeting() {
        return (
          <FormattedMessage
            defaultMessage="Hello!"
            description="Greeting message"
          />
        );
      }
    `;

    const sha1output = await transformCode(input, {
      idInterpolationPattern: "[sha1:contenthash:base64:6]",
    });
    const sha512output = await transformCode(input, {
      idInterpolationPattern: "[sha512:contenthash:base64:6]",
    });

    expect(sha1output).toMatch(/id: "[a-zA-Z0-9]{6}"/);
    expect(sha512output).toMatch(/id: "[a-zA-Z0-9]{6}"/);
    expect(sha1output).not.toMatch(sha512output);
  });
});

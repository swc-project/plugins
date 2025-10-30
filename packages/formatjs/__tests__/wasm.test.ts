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

  it("should handle string concatenation in defaultMessage", async () => {
    const input = `
      import { defineMessage } from 'react-intl';

      const message = defineMessage({
        defaultMessage: "Foo " + "Bar",
        description: "foobar"
      });
    `;

    const output = await transformCode(input);

    expect(output).toMatch(/id: "[^"]+"/);
    expect(output).toMatch(/defaultMessage: "Foo Bar"/);
    expect(output).not.toMatch(/description/);
  });

  it("should handle multiple string concatenations", async () => {
    const input = `
      import { defineMessage } from 'react-intl';

      const message = defineMessage({
        defaultMessage: "This is " + "a very " + "long message",
        description: "multi concat"
      });
    `;

    const output = await transformCode(input);

    expect(output).toMatch(/id: "[^"]+"/);
    expect(output).toMatch(/defaultMessage: "This is a very long message"/);
    expect(output).not.toMatch(/description/);
  });

  it("should handle string concatenation in FormattedMessage JSX", async () => {
    const input = `
      import React from 'react';
      import { FormattedMessage } from 'react-intl';

      export function Greeting() {
        return (
          <FormattedMessage
            defaultMessage={"Hello " + "World"}
            description="jsx concat"
          />
        );
      }
    `;

    const output = await transformCode(input);

    expect(output).toMatch(/id: "[^"]+"/);
    expect(output).toMatch(/defaultMessage: "Hello World"/);
    expect(output).not.toMatch(/description/);
  });

  it("should handle statically evaluate-able variables", async () => {
    const input = `
      import { defineMessage, formatMessage, FormattedMessage } from 'react-intl';

      const part1 = "Hello, ";
      const part2 = "world!";

      const message = defineMessage({
        defaultMessage: part1 + part2,
        description: "static vars"
      });
      function Greeting() {
        const message2 = formatMessage({
          defaultMessage: part1 + part2,
          description: "static vars in function"
        });
        const templateMessage = formatMessage({
          defaultMessage: \`\${part1}\${part2}\`,
          description: "static string"
        });
        return (<FormattedMessage defaultMessage={part1 + part2} />);
      }
    `;

    const output = await transformCode(input);
    expect(output).toMatchSnapshot();
  });

  it("should transform to ast when enabled", async () => {
    const input = `
      import { defineMessage, formatMessage, FormattedMessage } from 'react-intl';

      const helloWorldMessage = formatMessage({
        defaultMessage: "Hello, world!",
      });

      const helloWorld = defineMessage({
        defaultMessage: "Hello, world!",
        description: "A simple greeting",
      });

      export function Greeting() {
        return (
          <FormattedMessage defaultMessage="Hello, world!" />
        );
      }
    `;

    const code = await transformCode(input, { ast: true });

    expect(code).toMatchSnapshot();
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

    const md5output = await transformCode(input, {
      idInterpolationPattern: "[md5:contenthash:base64:6]",
    });
    const sha1output = await transformCode(input, {
      idInterpolationPattern: "[sha1:contenthash:base64:6]",
    });
    const sha512output = await transformCode(input, {
      idInterpolationPattern: "[sha512:contenthash:base64:6]",
    });

    expect(md5output).toMatch(/id: "[a-zA-Z0-9]{6}"/);
    expect(sha1output).toMatch(/id: "[a-zA-Z0-9]{6}"/);
    expect(sha512output).toMatch(/id: "[a-zA-Z0-9]{6}"/);
    expect(md5output).not.toMatch(sha512output);
    expect(sha1output).not.toMatch(sha512output);
  });

  it("should be able to use object description", async () => {
    const input = `
      import { FormattedMessage } from 'react-intl';

      export function Greeting() {
        return (
          <FormattedMessage
            defaultMessage="Hello!"
            description={{ text: "Greeting message" }}
          />
        );
      }
    `;

    const output = await transformCode(input);

    expect(output).toMatch(/id: "zL\/jyT"/);
  });

  it("should generate same id even if order of keys is different in two description objects with same keys", async () => {
    const input = `
      import { FormattedMessage } from 'react-intl';

      export function Greeting() {
        return (
          <FormattedMessage
            defaultMessage="Hello!"
            description={{ text: "Greeting message", image: "https://example.com/image.png" }}
          />
        );
      }
    `;

    const input2 = `
      import { FormattedMessage } from 'react-intl';

      export function Greeting() {
        return (
          <FormattedMessage
            defaultMessage="Hello!"
            description={{ image: "https://example.com/image.png", text: "Greeting message" }}
          />
        );
      }
    `;

    const output = await transformCode(input);
    const output2 = await transformCode(input2);

    expect(output).toMatch(output2);
  });

  it("should generate same id even if description is an external variable", async () => {
    const input = `
      import { FormattedMessage } from 'react-intl';

      const description = {
        text: "Hello description",
        img: "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png",
      };

      <FormattedMessage
        defaultMessage="Hello message {name}"
        description={description}
        values={{
          name: value.value,
        }}
      />
    `;

    const input2 = `
      import { FormattedMessage } from 'react-intl';

      export function Greeting() {
        return (
          <FormattedMessage
            defaultMessage="Hello message {name}"
            description={{
              text: "Hello description",
              img: "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png",
            }}
            values={{
              name: value.value,
            }}
          />
        );
      }
    `;

    const output = await transformCode(input);
    const output2 = await transformCode(input2);

    const id1 = output.match(/id: "([^"]+)"/)?.[1];
    const id2 = output2.match(/id: "([^"]+)"/)?.[1];

    expect(id1).toBe(id2);
  });

  it("should generate same id after react compiler optimizations", async () => {
    const input = `
      "use client";
      import { c as _c } from "react/compiler-runtime";

      import { FormattedMessage } from "react-intl";

      interface ClientProps {
        value: {
          value: string;
        };
      }

      export function Client(t0) {
        const $ = _c(3);
        const { value } = t0;
        let t1;
        if ($[0] === Symbol.for("react.memo_cache_sentinel")) {
          t1 = {
            text: "Hello",
            img: "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png",
          };
          $[0] = t1;
        } else {
          t1 = $[0];
        }
        let t2;
        if ($[1] !== value.value) {
          t2 = (
            <FormattedMessage
              defaultMessage="Hello {name}"
              description={t1}
              values={{ name: value.value }}
            />
          );
          $[1] = value.value;
          $[2] = t2;
        } else {
          t2 = $[2];
        }
        return t2;
      }
    `;

    const input2 = `
      import { FormattedMessage } from 'react-intl';

      export function Greeting() {
        return (
          <FormattedMessage
            defaultMessage="Hello {name}"
            description={{
              text: "Hello",
              img: "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png",
            }}
            values={{
              name: value.value,
            }}
          />
        );
      }
    `;

    const output = await transformCode(input);
    const output2 = await transformCode(input2);

    const id1 = output.match(/id: "([^"]+)"/)?.[1];
    const id2 = output2.match(/id: "([^"]+)"/)?.[1];

    expect(id1).toBe(id2);
  });

  it("should be able to use different encodings in interpolation", async () => {
    const input = `
      import { FormattedMessage } from 'react-intl';

      export function Greeting() {
        return (
          <FormattedMessage
            defaultMessage="Hello, World!"
            description="Greeting message"
          />
        );
      }
    `;

    const hexOutput = await transformCode(input, {
      idInterpolationPattern: "[sha512:contenthash:hex:9]",
    });

    const base64UrlOutput = await transformCode(input, {
      idInterpolationPattern: "[sha512:contenthash:base64url:12]",
    });

    expect(hexOutput).toMatch(/id: "[0-9a-f]{9}"/);
    expect(base64UrlOutput).toMatch(/id: "[a-zA-Z0-9-_]{12}"/);
  });

  it("should quote plural keys correctly when ast enabled", async () => {
    const input = `
      import { formatMessage } from 'react-intl';
      formatMessage(
        {
          defaultMessage: \`
            You did {count, plural,
              =0 {nothing}
              =1 {1 click}
              other {# clicks}
            }
          \`,
        },
        { count }
      )`;

    const code = await transformCode(input, { ast: true });

    expect(code).toMatchSnapshot();
  });

  it("should generate same id even if description is an template literal string", async () => {
    const input1 = `
      import { FormattedMessage } from 'react-intl';

      export function Greeting() {
        return (
          <FormattedMessage
            defaultMessage="Hello, World!"
            description="Greeting message"
          />
        );
      }
    `;

    const input2 = `
      import { FormattedMessage } from 'react-intl';

      export function Greeting() {
        return (
          <FormattedMessage
            defaultMessage="Hello, World!"
            description={\`Greeting message\`}
          />
        );
      }
    `;

    const input3 = `
      import { FormattedMessage } from 'react-intl';

      const description = \`Greeting message\`;

      export function Greeting() {
        return (
          <FormattedMessage
            defaultMessage="Hello, World!"
            description={description}
          />
        );
      }
    `;

    const code1 = await transformCode(input1, {
      idInterpolationPattern: "[sha512:contenthash:base64:6]",
    });
    const code2 = await transformCode(input2, {
      idInterpolationPattern: "[sha512:contenthash:base64:6]",
    });
    const code3 = await transformCode(input3, {
      idInterpolationPattern: "[sha512:contenthash:base64:6]",
    });

    expect(code1).toMatch(/id: "Ae\/S0P"/);
    expect(code2).toMatch(/id: "Ae\/S0P"/);
    expect(code3).toMatch(/id: "Ae\/S0P"/);
  });
});

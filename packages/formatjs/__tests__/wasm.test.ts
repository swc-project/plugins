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

  // Regression test for https://github.com/swc-project/plugins/issues/532
  it("should handle string concatenation in defaultMessage (issue #532)", async () => {
    const input = `
      import { defineMessage } from 'react-intl';

      const message = defineMessage({
        defaultMessage: 'Hello ' + 'world'
      });
    `;

    const output = await transformCode(input, { ast: true });

    expect(output).toMatch(/id: "[^"]+"/);
    // In AST mode, defaultMessage should be a non-empty ICU AST array.
    expect(output).toMatch(/defaultMessage: \[/);
    expect(output).not.toMatch(/defaultMessage: \[\]/);
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

      const part1 = "Hello";
      const part2 = "world";

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
          defaultMessage: \`~\${part1}, \${part2}!\`,
          description: "static string"
        });
        return (<FormattedMessage defaultMessage={part1 + part2} />);
      }
    `;

    const output = await transformCode(input);
    expect(output).toMatchSnapshot();
  });

  it("should throw error on non-statically evaluate-able variables", async () => {
    const input = `
      import { defineMessage, formatMessage, FormattedMessage } from 'react-intl';

      const part1 = "Hello, ";

      const message = defineMessage({
        defaultMessage: part1 + part2,
        description: "static vars"
      });
    `;

    await expect(transformCode(input)).rejects.toThrow(
      "[React Intl] Messages must be statically evaluate-able for extraction.",
    );
  });

  it("should throw by default on non-static descriptor ids", async () => {
    const input = `
      import { formatMessage } from 'react-intl';

      formatMessage({ id: backendProvidedId });
    `;

    await expect(transformCode(input)).rejects.toThrow(
      "[React Intl] Messages must be statically evaluate-able for extraction.",
    );
  });

  it("should skip non-static descriptors when throws is false", async () => {
    const input = `
      import React from 'react';
      import { FormattedMessage, defineMessages, useIntl } from 'react-intl';

      defineMessages({
        staticMessage: {
          defaultMessage: 'Static defineMessages message',
          description: 'Static defineMessages description',
        },
        dynamicId: {
          id: window.location.hash,
          defaultMessage: 'Dynamic defineMessages id',
        },
      });

      export function Example({ status }) {
        const intl = useIntl();

        return (
          <div>
            {intl.formatMessage({
              defaultMessage: 'Static formatMessage message',
              description: 'Static formatMessage description',
            })}
            {intl.formatMessage({
              id: status,
            })}
            {intl.formatMessage({
              defaultMessage: getDynamicMessage(),
            })}
            {intl.formatMessage({
              defaultMessage: intl.formatMessage({
                defaultMessage: 'Nested static formatMessage message',
                description: 'Nested static formatMessage description',
              }),
            })}
            <FormattedMessage
              defaultMessage="Static JSX message"
              description="Static JSX description"
            />
            <FormattedMessage
              id={\`Agent.Details.Status.\${status}\`}
              defaultMessage="Dynamic JSX id"
            />
          </div>
        );
      }
    `;

    const output = await transformCode(input, {
      ast: true,
      throws: false,
    });

    expect(output).toContain("id: window.location.hash");
    expect(output).toContain("id: status");
    expect(output).toContain("defaultMessage: getDynamicMessage()");
    expect(output).toContain("Agent.Details.Status.");
    expect(output).toContain('defaultMessage: "Dynamic JSX id"');
    expect(output.match(/id: "[A-Za-z0-9+/]{6}"/g)).toHaveLength(4);
    expect(output.match(/defaultMessage: \[/g)).toHaveLength(4);
  });

  it("should skip ICU parse errors when throws is false", async () => {
    const input = `
      import { FormattedMessage } from 'react-intl';

      export function Example() {
        return (
          <>
            <FormattedMessage defaultMessage="{count, plural, one {One}}" />
            <FormattedMessage defaultMessage="Static after parse error" />
          </>
        );
      }
    `;

    const output = await transformCode(input, {
      ast: true,
      throws: false,
    });

    expect(output).toContain('defaultMessage: "{count, plural, one {One}}"');
    expect(output.match(/id: "[A-Za-z0-9+/]{6}"/g)).toHaveLength(1);
    expect(output.match(/defaultMessage: \[/g)).toHaveLength(1);
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

  it("should not evaluate unrelated TypeScript and JSX code (issue #604)", async () => {
    const input = `
      "use client";

      let config: { info: { x: string } | null } = { info: null };

      export function setConfig(c: typeof config) {
        config = c;
      }

      function getParams() {
        const p: { x?: string } = {};
        if (config.info) {
          p.x = config.info.x;
        }
        return p;
      }

      function run() {
        console.log(getParams());
      }

      export default function Page() {
        return <div onClick={() => run()}>click</div>;
      }
    `;

    const output = await transformCode(input, { ast: true });

    expect(output).toContain("config.info.x");
    expect(output).toContain("onClick");
  });

  it("should transform several formatMessage calls with ast enabled (issue #604)", async () => {
    const input = `
      "use client";
      import { useIntl, IntlProvider } from "react-intl";

      function Messages() {
        const intl = useIntl();
        return (
          <ul>
            <li>{intl.formatMessage({ id: "a", defaultMessage: "Message A" })}</li>
            <li>{intl.formatMessage({ id: "b", defaultMessage: "Message B" })}</li>
            <li>{intl.formatMessage({ id: "c", defaultMessage: "Message C" })}</li>
            <li>{intl.formatMessage({ id: "d", defaultMessage: "Message D" })}</li>
            <li>{intl.formatMessage({ id: "e", defaultMessage: "Message E" })}</li>
          </ul>
        );
      }

      export default function Page() {
        return (
          <IntlProvider locale="en" messages={{}}>
            <Messages />
          </IntlProvider>
        );
      }
    `;

    const output = await transformCode(input, { ast: true });

    expect(output.match(/defaultMessage: \[/g)).toHaveLength(5);
  });

  it("should evaluate delayed bindings, member lookups, and primitive coercion", async () => {
    const input = `
      import { defineMessage, formatMessage } from 'react-intl';

      function laterBinding() {
        return formatMessage({
          defaultMessage: MSG,
        });
      }

      const id = "coerced";
      const suffix = true;
      const messages = {
        hello: "Hello from object",
      };

      defineMessage({
        id,
        defaultMessage: ("Value " + suffix) as const,
      });

      formatMessage({
        defaultMessage: messages.hello,
      });

      formatMessage({
        defaultMessage: \`Step \${2}\`,
      });

      formatMessage({
        defaultMessage: "Count " + (1 + true),
      });

      formatMessage({
        defaultMessage: \`Negative \${-1}\`,
      });

      formatMessage({
        defaultMessage: "Signed " + -1,
      });

      formatMessage({
        defaultMessage: true ? "Enabled" : "Disabled",
      });

      formatMessage({
        defaultMessage: \`ID \${1e21}\`,
      });

      const MSG = "Declared later";
    `;

    const output = await transformCode(input);

    expect(output).toMatch(/id: "coerced"/);
    expect(output).toContain('"Value true"');
    expect(output).toMatch(/defaultMessage: "Hello from object"/);
    expect(output).toMatch(/defaultMessage: "Step 2"/);
    expect(output).toMatch(/defaultMessage: "Count 2"/);
    expect(output).toMatch(/defaultMessage: "Negative -1"/);
    expect(output).toMatch(/defaultMessage: "Signed -1"/);
    expect(output).toMatch(/defaultMessage: "Enabled"/);
    expect(output).toMatch(/defaultMessage: "ID 1e\+21"/);
    expect(output).toMatch(/defaultMessage: "Declared later"/);
  });

  it("should ignore unknown shorthand props without evaluating them", async () => {
    const input = `
      import { defineMessage } from 'react-intl';

      defineMessage({
        defaultMessage: "Hello",
        metadata,
      });
    `;

    const output = await transformCode(input);

    expect(output).toMatch(/defaultMessage: "Hello"/);
    expect(output).toContain("metadata");
  });

  it("should remove shorthand defaultMessage when requested", async () => {
    const input = `
      import { defineMessage } from 'react-intl';

      const defaultMessage = "Hello";

      defineMessage({
        defaultMessage,
      });
    `;

    const output = await transformCode(input, { removeDefaultMessage: true });

    expect(output).toMatch(/defineMessage\(\{\s*id: "[^"]+"\s*\}\)/s);
  });

  it("should not resolve member values hidden behind later spreads", async () => {
    const input = `
      import { formatMessage } from 'react-intl';

      const runtimeMessages = {};
      const messages = {
        hello: "Hello",
        ...runtimeMessages,
      };

      formatMessage({
        defaultMessage: messages.hello,
      });
    `;

    await expect(transformCode(input)).rejects.toThrow(
      "[React Intl] Messages must be statically evaluate-able for extraction.",
    );
  });

  it("should not resolve member values hidden behind unknown computed keys", async () => {
    const input = `
      import { formatMessage } from 'react-intl';

      const runtimeKey = globalThis.key;
      const messages = {
        hello: "Hello",
        [runtimeKey]: "Runtime",
      };

      formatMessage({
        defaultMessage: messages.hello,
      });
    `;

    await expect(transformCode(input)).rejects.toThrow(
      "[React Intl] Messages must be statically evaluate-able for extraction.",
    );
  });

  it("should treat non-object reassignment as dynamic", async () => {
    const input = `
      import { formatMessage } from 'react-intl';

      let messages = {
        hello: "Hello",
      };
      messages = getMessages();

      formatMessage({
        defaultMessage: messages.hello,
      });
    `;

    await expect(transformCode(input)).rejects.toThrow(
      "[React Intl] Messages must be statically evaluate-able for extraction.",
    );
  });

  it("should preserve statement order for var redeclarations", async () => {
    const input = `
      import { formatMessage } from 'react-intl';

      var MSG = "First";

      formatMessage({
        defaultMessage: MSG,
      });

      var MSG = "Second";
    `;

    const output = await transformCode(input);

    expect(output).toMatch(/defaultMessage: "First"/);
    expect(output).not.toMatch(/defaultMessage: "Second"/);
  });

  it("should invalidate object bindings on member writes", async () => {
    const input = `
      import { formatMessage } from 'react-intl';

      const messages = {
        hello: "Hello",
      };
      messages.hello = getMessage();

      formatMessage({
        defaultMessage: messages.hello,
      });
    `;

    await expect(transformCode(input)).rejects.toThrow(
      "[React Intl] Messages must be statically evaluate-able for extraction.",
    );
  });

  it("should precollect bindings inside function bodies", async () => {
    const input = `
      import { FormattedMessage } from 'react-intl';

      function Component() {
        const render = () => <FormattedMessage defaultMessage={MSG} />;
        const MSG = "Hello from function";

        return render();
      }
    `;

    const output = await transformCode(input);

    expect(output).toMatch(/id: "[^"]+"/);
    expect(output).toMatch(/defaultMessage: MSG/);
  });

  it("should strip TypeScript wrappers while evaluating descriptor values", async () => {
    const input = `
      import { defineMessage } from 'react-intl';

      defineMessage({
        id: "wrapped" as const,
        defaultMessage: ("Wrapped message" satisfies string),
      });
    `;

    const output = await transformCode(input);

    expect(output).toMatch(/id: "wrapped"/);
    expect(output).toMatch(/defaultMessage: "Wrapped message"/);
  });

  it("should stop resolving cyclic bindings", async () => {
    const input = `
      import { defineMessage } from 'react-intl';

      let a = b;
      let b = a;

      defineMessage({
        defaultMessage: a,
      });
    `;

    await expect(transformCode(input)).rejects.toThrow(
      "[React Intl] Messages must be statically evaluate-able for extraction.",
    );
  });

  it("should not error on valid JSX outside formatjs calls (issue #588)", async () => {
    // Member expression JSX names like React.Suspense with JSX fallback props
    // should not trigger the static evaluation error.
    const input = `
      import React from 'react';

      const Loading = () => <div>Loading...</div>;

      function App() {
        return (
          <React.Suspense fallback={<Loading />}>
            <div>Content</div>
          </React.Suspense>
        );
      }
    `;

    // Should succeed without throwing "must be statically evaluate-able" error
    await expect(transformCode(input)).resolves.toBeDefined();
  });

  it("should transform member-expression FormattedMessage components", async () => {
    const input = `
      import React from 'react';
      import * as ReactIntl from 'react-intl';

      export function Greeting() {
        return (
          <ReactIntl.FormattedMessage
            defaultMessage="Hello, world!"
            description="Greeting message"
          />
        );
      }
    `;

    const output = await transformCode(input);

    expect(output).toMatch(/id: "[^"]+"/);
    expect(output).toMatch(/defaultMessage: "Hello, world!"/);
    expect(output).not.toMatch(/description/);
  });

  it("should not error on conditional JSX outside formatjs calls", async () => {
    // Conditional JSX expressions unrelated to formatjs should not be evaluated
    const input = `
      import React from 'react';

      function App({ isLoading }: { isLoading: boolean }) {
        return (
          <div>
            {isLoading ? <span>Loading...</span> : <span>Done</span>}
          </div>
        );
      }
    `;

    await expect(transformCode(input)).resolves.toBeDefined();
  });

  it("should not error on FormattedMessage with JSX values prop", async () => {
    // FormattedMessage with a `values` prop containing JSX should not error,
    // since `values` is not a known formatjs descriptor key.
    const input = `
      import React from 'react';
      import { FormattedMessage } from 'react-intl';

      function App() {
        return (
          <FormattedMessage
            defaultMessage="Hello <b>{name}</b>"
            values={{ b: (chunks) => <b>{chunks}</b>, name: "World" }}
          />
        );
      }
    `;

    const output = await transformCode(input);
    expect(output).toMatch(/defaultMessage/);
  });
});

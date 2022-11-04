import fs from 'fs'
import path from 'path'

import { run, html, css } from './util/run'

test('prefix', () => {
  let config = {
    prefix: 'tw-',
    darkMode: 'class',
    content: [path.resolve(__dirname, './prefix.test.html')],
    corePlugins: { preflight: false },
    theme: {
      animation: {
        spin: 'spin 1s linear infinite',
        ping: 'ping 1s cubic-bezier(0, 0, 0.2, 1) infinite',
      },
      keyframes: {
        spin: { to: { transform: 'rotate(360deg)' } },
      },
    },
    plugins: [
      function ({ addComponents, addUtilities }) {
        addComponents({
          '.btn-prefix': {
            button: 'yes',
          },
        })
        addComponents(
          {
            '.btn-no-prefix': {
              button: 'yes',
            },
          },
          { respectPrefix: false }
        )
        addUtilities({
          '.custom-util-prefix': {
            button: 'no',
          },
        })
        addUtilities(
          {
            '.custom-util-no-prefix': {
              button: 'no',
            },
          },
          { respectPrefix: false }
        )
      },
    ],
  }

  let input = css`
    @tailwind base;
    @tailwind components;
    @layer components {
      .custom-component {
        @apply tw-font-bold dark:group-hover:tw-font-normal;
      }
    }
    @tailwind utilities;
    @layer utilities {
      .custom-utility {
        foo: bar;
      }
    }
  `

  return run(input, config).then((result) => {
    let expectedPath = path.resolve(__dirname, './prefix.test.css')
    let expected = fs.readFileSync(expectedPath, 'utf8')

    expect(result.css).toMatchFormattedCss(expected)
  })
})

it('negative values: marker before prefix', async () => {
  let config = {
    prefix: 'tw-',
    content: [{ raw: html`<div class="-tw-top-1"></div>` }],
    corePlugins: { preflight: false },
  }

  let input = css`
    @tailwind utilities;
  `

  await run(input, config)

  const result = await run(input, config)

  expect(result.css).toMatchFormattedCss(css`
    .-tw-top-1 {
      top: -0.25rem;
    }
  `)
})

it('negative values: marker after prefix', async () => {
  let config = {
    prefix: 'tw-',
    content: [{ raw: html`<div class="tw--top-1"></div>` }],
    corePlugins: { preflight: false },
  }

  let input = css`
    @tailwind utilities;
  `

  await run(input, config)

  const result = await run(input, config)

  expect(result.css).toMatchFormattedCss(css`
    .tw--top-1 {
      top: -0.25rem;
    }
  `)
})

it('negative values: marker before prefix and arbitrary value', async () => {
  let config = {
    prefix: 'tw-',
    content: [{ raw: html`<div class="-tw-top-[1px]"></div>` }],
    corePlugins: { preflight: false },
  }

  let input = css`
    @tailwind utilities;
  `

  await run(input, config)

  const result = await run(input, config)

  expect(result.css).toMatchFormattedCss(css`
    .-tw-top-\[1px\] {
      top: -1px;
    }
  `)
})

it('negative values: marker after prefix and arbitrary value', async () => {
  let config = {
    prefix: 'tw-',
    content: [{ raw: html`<div class="tw--top-[1px]"></div>` }],
    corePlugins: { preflight: false },
  }

  let input = css`
    @tailwind utilities;
  `

  await run(input, config)

  const result = await run(input, config)

  expect(result.css).toMatchFormattedCss(css`
    .tw--top-\[1px\] {
      top: -1px;
    }
  `)
})

it('negative values: no marker and arbitrary value', async () => {
  let config = {
    prefix: 'tw-',
    content: [{ raw: html`<div class="tw-top-[-1px]"></div>` }],
    corePlugins: { preflight: false },
  }

  let input = css`
    @tailwind utilities;
  `

  await run(input, config)

  const result = await run(input, config)

  expect(result.css).toMatchFormattedCss(css`
    .tw-top-\[-1px\] {
      top: -1px;
    }
  `)
})

it('negative values: variant versions', async () => {
  let config = {
    prefix: 'tw-',
    content: [
      {
        raw: html`
          <div class="hover:-tw-top-1 hover:tw--top-1"></div>
          <div class="hover:-tw-top-[1px] hover:tw--top-[1px]"></div>
          <div class="hover:tw-top-[-1px]"></div>

          <!-- this one should not generate anything -->
          <div class="-hover:tw-top-1"></div>
        `,
      },
    ],
    corePlugins: { preflight: false },
  }

  let input = css`
    @tailwind utilities;
  `

  await run(input, config)

  const result = await run(input, config)

  expect(result.css).toMatchFormattedCss(css`
    .hover\:-tw-top-1:hover {
      top: -0.25rem;
    }
    .hover\:tw--top-1:hover {
      top: -0.25rem;
    }
    .hover\:-tw-top-\[1px\]:hover {
      top: -1px;
    }
    .hover\:tw--top-\[1px\]:hover {
      top: -1px;
    }
    .hover\:tw-top-\[-1px\]:hover {
      top: -1px;
    }
  `)
})

it('negative values: prefix and apply', async () => {
  let config = {
    prefix: 'tw-',
    content: [{ raw: html`` }],
    corePlugins: { preflight: false },
  }

  let input = css`
    @tailwind utilities;

    .a {
      @apply hover:tw--top-1;
    }
    .b {
      @apply hover:-tw-top-1;
    }
    .c {
      @apply hover:-tw-top-[1px];
    }
    .d {
      @apply hover:tw--top-[1px];
    }
    .e {
      @apply hover:tw-top-[-1px];
    }
  `

  await run(input, config)

  const result = await run(input, config)

  expect(result.css).toMatchFormattedCss(css`
    .a:hover {
      top: -0.25rem;
    }
    .b:hover {
      top: -0.25rem;
    }
    .c:hover {
      top: -1px;
    }
    .d:hover {
      top: -1px;
    }
    .e:hover {
      top: -1px;
    }
  `)
})

it('negative values: prefix in the safelist', async () => {
  let config = {
    prefix: 'tw-',
    safelist: [{ pattern: /-tw-top-1/g }, { pattern: /tw--top-1/g }],
    theme: {
      inset: {
        1: '0.25rem',
      },
    },
    content: [{ raw: html`` }],
    corePlugins: { preflight: false },
  }

  let input = css`
    @tailwind utilities;
  `

  await run(input, config)

  const result = await run(input, config)

  expect(result.css).toMatchFormattedCss(css`
    .-tw-top-1 {
      top: -0.25rem;
    }
    .tw--top-1 {
      top: -0.25rem;
    }
  `)
})

it('prefix with negative values and variants in the safelist', async () => {
  let config = {
    prefix: 'tw-',
    safelist: [
      { pattern: /-tw-top-1/, variants: ['hover', 'sm:hover'] },
      { pattern: /tw--top-1/, variants: ['hover', 'sm:hover'] },
    ],
    theme: {
      inset: {
        1: '0.25rem',
      },
    },
    content: [{ raw: html`` }],
    corePlugins: { preflight: false },
  }

  let input = css`
    @tailwind utilities;
  `

  await run(input, config)

  const result = await run(input, config)

  expect(result.css).toMatchFormattedCss(css`
    .-tw-top-1 {
      top: -0.25rem;
    }
    .tw--top-1 {
      top: -0.25rem;
    }
    .hover\:-tw-top-1:hover {
      top: -0.25rem;
    }

    .hover\:tw--top-1:hover {
      top: -0.25rem;
    }
    @media (min-width: 640px) {
      .sm\:hover\:-tw-top-1:hover {
        top: -0.25rem;
      }
      .sm\:hover\:tw--top-1:hover {
        top: -0.25rem;
      }
    }
  `)
})

it('prefix does not detect and generate unnecessary classes', async () => {
  let config = {
    prefix: 'tw-_',
    content: [{ raw: html`-aaa-filter aaaa-table aaaa-hidden` }],
    corePlugins: { preflight: false },
  }

  let input = css`
    @tailwind utilities;
  `

  const result = await run(input, config)

  expect(result.css).toMatchFormattedCss(css``)
})

it('supports prefixed utilities using arbitrary values', async () => {
  let config = {
    prefix: 'tw-',
    content: [{ raw: html`foo` }],
    corePlugins: { preflight: false },
  }

  let input = css`
    .foo {
      @apply tw-text-[color:rgb(var(--button-background,var(--primary-button-background)))];
      @apply tw-ease-[cubic-bezier(0.77,0,0.175,1)];
      @apply tw-rounded-[min(4px,var(--input-border-radius))];
    }
  `

  const result = await run(input, config)

  expect(result.css).toMatchFormattedCss(css`
    .foo {
      color: rgb(var(--button-background, var(--primary-button-background)));
      transition-timing-function: cubic-bezier(0.77, 0, 0.175, 1);
      border-radius: min(4px, var(--input-border-radius));
    }
  `)
})

it('supports non-word prefixes (1)', async () => {
  let config = {
    prefix: '@',
    content: [
      {
        raw: html`
          <div class="@underline"></div>
          <div class="@bg-black"></div>
          <div class="@[color:red]"></div>
          <div class="hover:before:@content-['Hovering']"></div>
          <div class="my-utility"></div>
          <div class="foo"></div>

          <!-- these won't be detected -->
          <div class="overline"></div>
        `,
      },
    ],
    corePlugins: { preflight: false },
  }

  let input = css`
    @tailwind utilities;
    @layer utilities {
      .my-utility {
        color: orange;
      }
    }
    .foo {
      @apply @text-white;
      @apply [background-color:red];
    }
  `

  const result = await run(input, config)

  expect(result.css).toMatchFormattedCss(css`
    .\@bg-black {
      --tw-bg-opacity: 1;
      background-color: rgb(0 0 0 / var(--tw-bg-opacity));
    }
    .\@underline {
      text-decoration-line: underline;
    }
    .my-utility {
      color: orange;
    }
    .foo {
      --tw-text-opacity: 1;
      color: rgb(255 255 255 / var(--tw-text-opacity));
      background-color: red;
    }
    .hover\:before\:\@content-\[\'Hovering\'\]:hover::before {
      --tw-content: 'Hovering';
      content: var(--tw-content);
    }
  `)
})

it('supports non-word prefixes (2)', async () => {
  let config = {
    prefix: '@]$',
    content: [
      {
        raw: html`
          <div class="@]$underline"></div>
          <div class="@]$bg-black"></div>
          <div class="@]$[color:red]"></div>
          <div class="hover:before:@]$content-['Hovering']"></div>
          <div class="my-utility"></div>
          <div class="foo"></div>

          <!-- these won't be detected -->
          <div class="overline"></div>
        `,
      },
    ],
    corePlugins: { preflight: false },
  }

  let input = css`
    @tailwind utilities;
    @layer utilities {
      .my-utility {
        color: orange;
      }
    }
    .foo {
      @apply @]$text-white;
      @apply [background-color:red];
    }
  `

  const result = await run(input, config)

  // TODO: The class `.hover\:before\:\@\]\$content-\[\'Hovering\'\]:hover::before` is not generated
  // This happens because of the parenthesis/brace/bracket clipping performed on candidates

  expect(result.css).toMatchFormattedCss(css`
    .\@\]\$bg-black {
      --tw-bg-opacity: 1;
      background-color: rgb(0 0 0 / var(--tw-bg-opacity));
    }
    .\@\]\$underline {
      text-decoration-line: underline;
    }
    .my-utility {
      color: orange;
    }
    .foo {
      --tw-text-opacity: 1;
      color: rgb(255 255 255 / var(--tw-text-opacity));
      background-color: red;
    }
  `)
})

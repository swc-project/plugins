// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/tagged-template-args-forwarded.js

import { css } from "@emotion/react";

function media(...args) {
  return css`
    @media (min-width: 100px) {
      ${css(...args)};
    }
  `;
}

const test = css`
  ${media`color: red;`};
`;

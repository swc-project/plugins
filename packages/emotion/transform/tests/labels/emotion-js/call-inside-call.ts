// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/call-inside-call.js

import { css } from "@emotion/react";

const thing = css`
  display: flex;
  &:hover {
    ${css`
      color: hotpink;
    `};
  }
`;

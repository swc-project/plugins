// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/label-function-expression-named.js

import { css } from "@emotion/react";

const thing = function someName() {
  return css`
    color: hotpink;
  `;
};

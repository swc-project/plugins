// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/no-label-array-pattern.js

import { css } from "@emotion/react";

const [weirdo] = [
  css`
    color: hotpink;
  `,
];

export default weirdo;

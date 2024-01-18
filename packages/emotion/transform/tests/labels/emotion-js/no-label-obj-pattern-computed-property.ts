// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/no-label-obj-pattern-computed-property.js

import { css } from "@emotion/react";

const computed = "weirdo";

const { weirdo } = {
  [computed]: css`
    color: hotpink;
  `,
};

export default weirdo;

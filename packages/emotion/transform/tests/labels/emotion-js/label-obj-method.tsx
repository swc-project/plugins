// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/label-obj-method.js

import { css } from "@emotion/react";

const obj = {
  FooBar() {
    return (
      <div
        css={css`
          background-color: hotpink;
        `}
      />
    );
  },
};

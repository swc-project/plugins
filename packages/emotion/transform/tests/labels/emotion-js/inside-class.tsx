// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/inside-class.js

import { Component } from "react";
import { jsx, css } from "@emotion/react";

class SomeComponent extends Component {
  member = css`
    color: hotpink;
  `;

  render() {
    return (
      <div
        css={css`
          color: hotpink;
        `}
      />
    );
  }
}

class SomeOtherComponent extends Component {
  member = css`
    color: hotpink;
  `;

  render() {
    return (
      <div
        css={css`
          color: hotpink;

          .foo {
            ${this.member}
          }
        `}
      />
    );
  }
}

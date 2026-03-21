import { css } from "@emotion/react";
import { PureComponent } from "react";

export class SimpleComponent extends PureComponent {
  render() {
    return (
      <div css={{ color: "red" }}>Hello</div>
    );
  }
}

// Also test with array of objects
export function ArrayCssProp() {
  return <div css={[{ color: "blue" }, { background: "white" }]}>World</div>;
}

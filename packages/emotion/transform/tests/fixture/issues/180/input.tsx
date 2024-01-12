import { css } from "@emotion/react";
import styled from "@emotion/styled";

// Example 1
function myStyled(Component) {
  return styled(Component)`
    background-color: red;
  `;
}

function myCss(color) {
  return css`
    background-color: ${color};
  `;
}

const myStyles = myCss("red");

const Div = myStyled("div");

function App() {
  return (
    <>
      <Div>one</Div>
      <div css={myStyles}>two</div>
    </>
  );
}

// Example 2
const styles = {
  keyA: css({
    padding: 0,
  }),
  keyB: css({
    margin: 0,
  }),
};
const App2 = () => <div classname={styles.keyA}>hello world</div>;

import { css } from "@emotion/react";

const styles = {
  keyA: css({
    padding: 0,
  }),
  keyB: css({
    margin: 0,
  }),
};
const App = () => <div classname={styles.keyA}>hello world</div>;

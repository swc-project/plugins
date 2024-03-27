import css from "styled-jsx/css";

const { className: cardClassName, styles } = css.resolve`
  :hover {
    z-index: ${hoverAnimation ? "1" : "auto"};
  }
`;

import _JSXStyle from "styled-jsx/style";
const MOBILE_MAX = 767;
export default function Home() {
  return (
    <div className={_JSXStyle.dynamic([["8ec558d4d19955bb", [MOBILE_MAX]]])}>
      <h1
        className={
          _JSXStyle.dynamic([["8ec558d4d19955bb", [MOBILE_MAX]]]) +
          " " +
          "header"
        }
      >
        Hello
      </h1>

      <_JSXStyle
        id={"8ec558d4d19955bb"}
        dynamic={[MOBILE_MAX]}
      >{`.header.__jsx-style-dynamic-selector{font-size:48px}`}</_JSXStyle>
    </div>
  );
}

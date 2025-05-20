import _JSXStyle from "styled-jsx/style";
const MOBILE_MAX = 767;
export default function Home() {
    return <div className={_JSXStyle.dynamic([
        [
            "50a990c962011b6e",
            [
                MOBILE_MAX
            ]
        ]
    ])}>
      <h1 className={_JSXStyle.dynamic([
        [
            "50a990c962011b6e",
            [
                MOBILE_MAX
            ]
        ]
    ]) + " " + "header"}>Hello</h1>
      <_JSXStyle id={"50a990c962011b6e"} dynamic={[
        MOBILE_MAX
    ]}>{`.header.__jsx-style-dynamic-selector{font-size:48px}@media screen and (max-width:${MOBILE_MAX}px){.header.__jsx-style-dynamic-selector{font-size:12px}}`}</_JSXStyle>
    </div>;
}

import _JSXStyle from "styled-jsx/style";
const MOBILE_MAX = 767;
export default function Home() {
    return <div className={_JSXStyle.dynamic([
        [
            "8a4dbea836af7ee3",
            [
                MOBILE_MAX
            ]
        ]
    ])}>
      <h1 className={_JSXStyle.dynamic([
        [
            "8a4dbea836af7ee3",
            [
                MOBILE_MAX
            ]
        ]
    ]) + " " + "header"}>Hello</h1>
      <_JSXStyle id={"8a4dbea836af7ee3"} dynamic={[
        MOBILE_MAX
    ]}>{`.header.__jsx-style-dynamic-selector{font-size:48px}@media screen and (max-width:${MOBILE_MAX}px){.header.__jsx-style-dynamic-selector{font-size:12px}}`}</_JSXStyle>
    </div>;
}

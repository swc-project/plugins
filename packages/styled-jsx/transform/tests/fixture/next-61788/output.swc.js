import _JSXStyle from "styled-jsx/style";
const MOBILE_MAX = 767;
export default function Home() {
    return <div className={_JSXStyle.dynamic([
        [
            "b1e45794aa5a4947",
            [
                MOBILE_MAX
            ]
        ]
    ])}>
      <h1 className={_JSXStyle.dynamic([
        [
            "b1e45794aa5a4947",
            [
                MOBILE_MAX
            ]
        ]
    ]) + " " + "header"}>Hello</h1>
      <_JSXStyle id={"b1e45794aa5a4947"} dynamic={[
        MOBILE_MAX
    ]}>{`.header.__jsx-style-dynamic-selector{font-size:48px}@media screen and (width<=${MOBILE_MAX}px){.header.__jsx-style-dynamic-selector{font-size:12px}}`}</_JSXStyle>
    </div>;
}

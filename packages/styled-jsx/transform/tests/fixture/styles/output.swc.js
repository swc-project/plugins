import _JSXStyle from "styled-jsx/style";
import colors, { size } from "./constants";
const color = "red";
const bar = new String("div.jsx-aaed0341accea8f{font-size:3em}");
bar.__hash = "aaed0341accea8f";
const baz = new String("div{font-size:3em}");
baz.__hash = "aaed0341accea8f";
const a = new String(`div{font-size:${size}em}`);
a.__hash = "61d691964f127a91";
export const uh = bar;
export const foo = new String(`div.jsx-275572683a969625{color:${color}}`);
foo.__hash = "275572683a969625";
({
    styles: <_JSXStyle id={"5b9cd0220e690172"}>{`div.jsx-5b9cd0220e690172{color:${colors.green.light}}a.jsx-5b9cd0220e690172{color:red}`}</_JSXStyle>,
    className: "jsx-5b9cd0220e690172"
});
const b = {
    styles: <_JSXStyle id={"ad313abbd006874f"}>{`div.jsx-ad313abbd006874f{color:${colors.green.light}}a.jsx-ad313abbd006874f{color:red}`}</_JSXStyle>,
    className: "jsx-ad313abbd006874f"
};
const dynamic = (colors)=>{
    const b = {
        styles: <_JSXStyle id={"b563f3fa97e0844"} dynamic={[
            colors.green.light
        ]}>{`div.__jsx-style-dynamic-selector{color:${colors.green.light}}a.__jsx-style-dynamic-selector{color:red}`}</_JSXStyle>,
        className: _JSXStyle.dynamic([
            [
                "b563f3fa97e0844",
                [
                    colors.green.light
                ]
            ]
        ])
    };
};
export default {
    styles: <_JSXStyle id={"bd08cd6b79dfff27"}>{`div.jsx-bd08cd6b79dfff27{font-size:3em}p.jsx-bd08cd6b79dfff27{color:${color}}`}</_JSXStyle>,
    className: "jsx-bd08cd6b79dfff27"
};

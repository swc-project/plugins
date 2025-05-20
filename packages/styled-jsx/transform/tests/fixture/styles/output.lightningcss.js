import _JSXStyle from "styled-jsx/style";
import colors, { size } from "./constants";
const color = "red";
const bar = new String("div.jsx-aaed0341accea8f{font-size:3em}");
bar.__hash = "aaed0341accea8f";
const baz = new String("div{font-size:3em}");
baz.__hash = "aaed0341accea8f";
const a = new String(`div{font-size:${size}em}`);
a.__hash = "82ae9187e674a46";
export const uh = bar;
export const foo = new String(`div.jsx-e5612f479f7da76d{color:${color}}`);
foo.__hash = "e5612f479f7da76d";
({
    styles: <_JSXStyle id={"9d51d094e917d670"}>{`div.jsx-9d51d094e917d670{color:${colors.green.light}}a.jsx-9d51d094e917d670{color:red}`}</_JSXStyle>,
    className: "jsx-9d51d094e917d670"
});
const b = {
    styles: <_JSXStyle id={"b6a0beb223e5f47e"}>{`div.jsx-b6a0beb223e5f47e{color:${colors.green.light}}a.jsx-b6a0beb223e5f47e{color:red}`}</_JSXStyle>,
    className: "jsx-b6a0beb223e5f47e"
};
const dynamic = (colors)=>{
    const b = {
        styles: <_JSXStyle id={"e9a45dad91c20c5c"} dynamic={[
            colors.green.light
        ]}>{`div.__jsx-style-dynamic-selector{color:${colors.green.light}}a.__jsx-style-dynamic-selector{color:red}`}</_JSXStyle>,
        className: _JSXStyle.dynamic([
            [
                "e9a45dad91c20c5c",
                [
                    colors.green.light
                ]
            ]
        ])
    };
};
export default {
    styles: <_JSXStyle id={"f132fb7c41f2107a"}>{`div.jsx-f132fb7c41f2107a{font-size:3em}p.jsx-f132fb7c41f2107a{color:${color}}`}</_JSXStyle>,
    className: "jsx-f132fb7c41f2107a"
};

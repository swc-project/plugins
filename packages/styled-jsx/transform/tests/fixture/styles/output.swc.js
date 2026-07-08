import _JSXStyle from "styled-jsx/style";
import colors, { size } from "./constants";
const color = "red";
const bar = new String("div.jsx-aaed0341accea8f{font-size:3em}");
bar.__hash = "aaed0341accea8f";
const baz = new String("div{font-size:3em}");
baz.__hash = "aaed0341accea8f";
const a = new String(`div{font-size:${size}em}`);
a.__hash = "89fd04668ad7cf7d";
export const uh = bar;
export const foo = new String(`div.jsx-67f6a2a1a75d9d6c{color:${color}}`);
foo.__hash = "67f6a2a1a75d9d6c";
({
    styles: <_JSXStyle id={"11d335620742f83"}>{`div.jsx-11d335620742f83{color:${colors.green.light}}a.jsx-11d335620742f83{color:red}`}</_JSXStyle>,
    className: "jsx-11d335620742f83"
});
const b = {
    styles: <_JSXStyle id={"13b7060dd34f3f97"}>{`div.jsx-13b7060dd34f3f97{color:${colors.green.light}}a.jsx-13b7060dd34f3f97{color:red}`}</_JSXStyle>,
    className: "jsx-13b7060dd34f3f97"
};
const dynamic = (colors)=>{
    const b = {
        styles: <_JSXStyle id={"3aa9e22f55c362d3"} dynamic={[
            colors.green.light
        ]}>{`div.__jsx-style-dynamic-selector{color:${colors.green.light}}a.__jsx-style-dynamic-selector{color:red}`}</_JSXStyle>,
        className: _JSXStyle.dynamic([
            [
                "3aa9e22f55c362d3",
                [
                    colors.green.light
                ]
            ]
        ])
    };
};
export default {
    styles: <_JSXStyle id={"2ffe2d57b3f2d2a5"}>{`div.jsx-2ffe2d57b3f2d2a5{font-size:3em}p.jsx-2ffe2d57b3f2d2a5{color:${color}}`}</_JSXStyle>,
    className: "jsx-2ffe2d57b3f2d2a5"
};

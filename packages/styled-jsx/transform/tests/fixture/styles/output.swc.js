import _JSXStyle from "styled-jsx/style";
import colors, { size } from "./constants";
const color = "red";
const bar = new String("div.jsx-aaed0341accea8f{font-size:3em}");
bar.__hash = "aaed0341accea8f";
const baz = new String("div{font-size:3em}");
baz.__hash = "aaed0341accea8f";
const a = new String(`div{font-size:${size}em}`);
a.__hash = "8228e42d40d15965";
export const uh = bar;
export const foo = new String(`div.jsx-b43c82a70e4f0472{color:${color}}`);
foo.__hash = "b43c82a70e4f0472";
({
    styles: <_JSXStyle id={"1064355a76cd6647"}>{`div.jsx-1064355a76cd6647{color:${colors.green.light}}a.jsx-1064355a76cd6647{color:red}`}</_JSXStyle>,
    className: "jsx-1064355a76cd6647"
});
const b = {
    styles: <_JSXStyle id={"8301ec39281a1d5c"}>{`div.jsx-8301ec39281a1d5c{color:${colors.green.light}}a.jsx-8301ec39281a1d5c{color:red}`}</_JSXStyle>,
    className: "jsx-8301ec39281a1d5c"
};
const dynamic = (colors)=>{
    const b = {
        styles: <_JSXStyle id={"da01783efa8657c1"} dynamic={[
            colors.green.light
        ]}>{`div.__jsx-style-dynamic-selector{color:${colors.green.light}}a.__jsx-style-dynamic-selector{color:red}`}</_JSXStyle>,
        className: _JSXStyle.dynamic([
            [
                "da01783efa8657c1",
                [
                    colors.green.light
                ]
            ]
        ])
    };
};
export default {
    styles: <_JSXStyle id={"14be4d3687cb89a3"}>{`div.jsx-14be4d3687cb89a3{font-size:3em}p.jsx-14be4d3687cb89a3{color:${color}}`}</_JSXStyle>,
    className: "jsx-14be4d3687cb89a3"
};

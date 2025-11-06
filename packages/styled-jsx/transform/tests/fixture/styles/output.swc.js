import _JSXStyle from "styled-jsx/style";
import colors, { size } from "./constants";
const color = "red";
const bar = new String("div.jsx-aaed0341accea8f{font-size:3em}");
bar.__hash = "aaed0341accea8f";
const baz = new String("div{font-size:3em}");
baz.__hash = "aaed0341accea8f";
const a = new String(`div{font-size:${size}em}`);
a.__hash = "100f5368f115df32";
export const uh = bar;
export const foo = new String(`div.jsx-ef1a1cb26d811a9f{color:${color}}`);
foo.__hash = "ef1a1cb26d811a9f";
({
    styles: <_JSXStyle id={"bb171c68dcc082f0"}>{`div.jsx-bb171c68dcc082f0{color:${colors.green.light}}a.jsx-bb171c68dcc082f0{color:red}`}</_JSXStyle>,
    className: "jsx-bb171c68dcc082f0"
});
const b = {
    styles: <_JSXStyle id={"ade616ec874fd5eb"}>{`div.jsx-ade616ec874fd5eb{color:${colors.green.light}}a.jsx-ade616ec874fd5eb{color:red}`}</_JSXStyle>,
    className: "jsx-ade616ec874fd5eb"
};
const dynamic = (colors)=>{
    const b = {
        styles: <_JSXStyle id={"27362acd75575fc"} dynamic={[
            colors.green.light
        ]}>{`div.__jsx-style-dynamic-selector{color:${colors.green.light}}a.__jsx-style-dynamic-selector{color:red}`}</_JSXStyle>,
        className: _JSXStyle.dynamic([
            [
                "27362acd75575fc",
                [
                    colors.green.light
                ]
            ]
        ])
    };
};
export default {
    styles: <_JSXStyle id={"662317bd20d2c6c5"}>{`div.jsx-662317bd20d2c6c5{font-size:3em}p.jsx-662317bd20d2c6c5{color:${color}}`}</_JSXStyle>,
    className: "jsx-662317bd20d2c6c5"
};

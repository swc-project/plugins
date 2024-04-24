import _JSXStyle from "styled-jsx/style";
import colors, { size } from "./constants";
const color = "red";
const bar = new String("div.jsx-aaed0341accea8f{font-size:3em}");
bar.__hash = "aaed0341accea8f";
const baz = new String("div{font-size:3em}");
baz.__hash = "aaed0341accea8f";
const a = new String(`div{font-size:${size}em}`);
a.__hash = "191c4a18d2e37256";
export const uh = bar;
export const foo = new String(`div.jsx-4741e5e93407b746{color:${color}}`);
foo.__hash = "4741e5e93407b746";
({
    styles: <_JSXStyle id={"221225cea79e61ac"}>{`div.jsx-221225cea79e61ac{color:${colors.green.light}}a.jsx-221225cea79e61ac{color:red}`}</_JSXStyle>,
    className: "jsx-221225cea79e61ac"
});
const b = {
    styles: <_JSXStyle id={"d0a60866d0a6e507"}>{`div.jsx-d0a60866d0a6e507{color:${colors.green.light}}a.jsx-d0a60866d0a6e507{color:red}`}</_JSXStyle>,
    className: "jsx-d0a60866d0a6e507"
};
const dynamic = (colors)=>{
    const b = {
        styles: <_JSXStyle id={"16174681411c9a0d"} dynamic={[
            colors.green.light
        ]}>{`div.__jsx-style-dynamic-selector{color:${colors.green.light}}a.__jsx-style-dynamic-selector{color:red}`}</_JSXStyle>,
        className: _JSXStyle.dynamic([
            [
                "16174681411c9a0d",
                [
                    colors.green.light
                ]
            ]
        ])
    };
};
export default {
    styles: <_JSXStyle id={"cdd18b1c7798f128"}>{`div.jsx-cdd18b1c7798f128{font-size:3em}p.jsx-cdd18b1c7798f128{color:${color}}`}</_JSXStyle>,
    className: "jsx-cdd18b1c7798f128"
};

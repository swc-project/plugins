import _JSXStyle from "styled-jsx/style";
import colors, { size } from "./constants";
const color = "red";
const bar = new String("div.jsx-aaed0341accea8f{font-size:3em}");
bar.__hash = "aaed0341accea8f";
const baz = new String("div{font-size:3em}");
baz.__hash = "aaed0341accea8f";
const a = new String(`div{font-size:${size}em}`);
a.__hash = "85553c319e97df68";
export const uh = bar;
export const foo = new String(`div.jsx-dcbfb3d2e344f9db{color:${color}}`);
foo.__hash = "dcbfb3d2e344f9db";
({
    styles: <_JSXStyle id={"f88621f583535018"}>{`div.jsx-f88621f583535018{color:${colors.green.light}}a.jsx-f88621f583535018{color:red}`}</_JSXStyle>,
    className: "jsx-f88621f583535018"
});
const b = {
    styles: <_JSXStyle id={"fb291e2b3d27a1d7"}>{`div.jsx-fb291e2b3d27a1d7{color:${colors.green.light}}a.jsx-fb291e2b3d27a1d7{color:red}`}</_JSXStyle>,
    className: "jsx-fb291e2b3d27a1d7"
};
const dynamic = (colors)=>{
    const b = {
        styles: <_JSXStyle id={"9ed6e7e5d69c0416"} dynamic={[
            colors.green.light
        ]}>{`div.__jsx-style-dynamic-selector{color:${colors.green.light}}a.__jsx-style-dynamic-selector{color:red}`}</_JSXStyle>,
        className: _JSXStyle.dynamic([
            [
                "9ed6e7e5d69c0416",
                [
                    colors.green.light
                ]
            ]
        ])
    };
};
export default {
    styles: <_JSXStyle id={"d32a731c032e7c5b"}>{`div.jsx-d32a731c032e7c5b{font-size:3em}p.jsx-d32a731c032e7c5b{color:${color}}`}</_JSXStyle>,
    className: "jsx-d32a731c032e7c5b"
};

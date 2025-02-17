import _JSXStyle from "styled-jsx/style";
import colors, { size } from "./constants";
const color = "red";
const bar = new String("div.jsx-aaed0341accea8f{font-size:3em}");
bar.__hash = "aaed0341accea8f";
const baz = new String("div{font-size:3em}");
baz.__hash = "aaed0341accea8f";
const a = new String(`div{font-size:${size}em}`);
a.__hash = "cc8b368179309c83";
export const uh = bar;
export const foo = new String(`div.jsx-686d72be815ad61f{color:${color}}`);
foo.__hash = "686d72be815ad61f";
({
    styles: <_JSXStyle id={"cbf39e21ee0143b0"}>{`div.jsx-cbf39e21ee0143b0{color:${colors.green.light}}a.jsx-cbf39e21ee0143b0{color:red}`}</_JSXStyle>,
    className: "jsx-cbf39e21ee0143b0"
});
const b = {
    styles: <_JSXStyle id={"fb1d9786220830be"}>{`div.jsx-fb1d9786220830be{color:${colors.green.light}}a.jsx-fb1d9786220830be{color:red}`}</_JSXStyle>,
    className: "jsx-fb1d9786220830be"
};
const dynamic = (colors)=>{
    const b = {
        styles: <_JSXStyle id={"97dfba18bfe0b7ba"} dynamic={[
            colors.green.light
        ]}>{`div.__jsx-style-dynamic-selector{color:${colors.green.light}}a.__jsx-style-dynamic-selector{color:red}`}</_JSXStyle>,
        className: _JSXStyle.dynamic([
            [
                "97dfba18bfe0b7ba",
                [
                    colors.green.light
                ]
            ]
        ])
    };
};
export default {
    styles: <_JSXStyle id={"7e954f5b3a7ab7da"}>{`div.jsx-7e954f5b3a7ab7da{font-size:3em}p.jsx-7e954f5b3a7ab7da{color:${color}}`}</_JSXStyle>,
    className: "jsx-7e954f5b3a7ab7da"
};

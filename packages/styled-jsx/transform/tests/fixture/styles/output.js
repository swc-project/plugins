import _JSXStyle from "styled-jsx/style";
import colors, { size } from './constants';
const color = 'red';
const bar = new String("div.jsx-aaed0341accea8f{font-size:3em}");
bar.__hash = "aaed0341accea8f";
const baz = new String("div{font-size:3em}");
baz.__hash = "aaed0341accea8f";
const a = new String(`div{font-size:${size}em}`);
a.__hash = "b4e02b3e84cc50c";
export const uh = bar;
export const foo = new String(`div.jsx-1a001e3709d54ba4{color:${color}}`);
foo.__hash = "1a001e3709d54ba4";
({
    styles: <_JSXStyle id={"7a7480dd17e82e07"}>{`div.jsx-7a7480dd17e82e07{color:${colors.green.light}}a.jsx-7a7480dd17e82e07{color:red}`}</_JSXStyle>,
    className: "jsx-7a7480dd17e82e07"
});
const b = {
    styles: <_JSXStyle id={"7a7480dd17e82e07"}>{`div.jsx-7a7480dd17e82e07{color:${colors.green.light}}a.jsx-7a7480dd17e82e07{color:red}`}</_JSXStyle>,
    className: "jsx-7a7480dd17e82e07"
};
const dynamic = (colors1)=>{
    const b = {
        styles: <_JSXStyle id={"9f775f199a8dfb95"} dynamic={[
            colors1.green.light
        ]}>{`div.__jsx-style-dynamic-selector{color:${colors1.green.light}}a.__jsx-style-dynamic-selector{color:red}`}</_JSXStyle>,
        className: _JSXStyle.dynamic([
            [
                "9f775f199a8dfb95",
                [
                    colors1.green.light
                ]
            ]
        ])
    };
};
export default {
    styles: <_JSXStyle id={"e14aa5a1efa47449"}>{`div.jsx-e14aa5a1efa47449{font-size:3em}p.jsx-e14aa5a1efa47449{color:${color}}`}</_JSXStyle>,
    className: "jsx-e14aa5a1efa47449"
};

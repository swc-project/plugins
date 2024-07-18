import _JSXStyle from "styled-jsx/style";
import colors, { size } from "./constants";
const color = "red";
const bar = new String("div.jsx-aaed0341accea8f{font-size:3em}");
bar.__hash = "aaed0341accea8f";
const baz = new String("div{font-size:3em}");
baz.__hash = "aaed0341accea8f";
const a = new String(`div{font-size:${size}em}`);
a.__hash = "f3f709dce236d2c7";
export const uh = bar;
export const foo = new String(`div.jsx-58279132b4e97b93{color:${color}}`);
foo.__hash = "58279132b4e97b93";
({
    styles: <_JSXStyle id={"4616b980b67d37b1"}>{`div.jsx-4616b980b67d37b1{color:${colors.green.light}}a.jsx-4616b980b67d37b1{color:red}`}</_JSXStyle>,
    className: "jsx-4616b980b67d37b1"
});
const b = {
    styles: <_JSXStyle id={"1d9f9a0af736c476"}>{`div.jsx-1d9f9a0af736c476{color:${colors.green.light}}a.jsx-1d9f9a0af736c476{color:red}`}</_JSXStyle>,
    className: "jsx-1d9f9a0af736c476"
};
const dynamic = (colors)=>{
    const b = {
        styles: <_JSXStyle id={"bde9d329308be70b"} dynamic={[
            colors.green.light
        ]}>{`div.__jsx-style-dynamic-selector{color:${colors.green.light}}a.__jsx-style-dynamic-selector{color:red}`}</_JSXStyle>,
        className: _JSXStyle.dynamic([
            [
                "bde9d329308be70b",
                [
                    colors.green.light
                ]
            ]
        ])
    };
};
export default {
    styles: <_JSXStyle id={"2707c6e87ff4b8f7"}>{`div.jsx-2707c6e87ff4b8f7{font-size:3em}p.jsx-2707c6e87ff4b8f7{color:${color}}`}</_JSXStyle>,
    className: "jsx-2707c6e87ff4b8f7"
};

import _JSXStyle from "swc-magic/style";
import colors, { size } from './constants';
const color = 'red';
const bar = new String("div.jsx-aaed0341accea8f{font-size:3em}");
bar.__hash = "aaed0341accea8f";
const baz = new String("div{font-size:3em}");
baz.__hash = "aaed0341accea8f";
const a = new String(`div{font-size:${size}em}`);
a.__hash = "c82345d11d3b02f5";
export const uh = bar;
export const foo = new String(`div.jsx-611c1f2c95744b70{color:${color}}`);
foo.__hash = "611c1f2c95744b70";
({
    styles: <_JSXStyle id={"141b18386ca7cbc5"}>{`div.jsx-141b18386ca7cbc5{color:${colors.green.light}}a.jsx-141b18386ca7cbc5{color:red}`}</_JSXStyle>,
    className: "jsx-141b18386ca7cbc5"
});
const b = {
    styles: <_JSXStyle id={"141b18386ca7cbc5"}>{`div.jsx-141b18386ca7cbc5{color:${colors.green.light}}a.jsx-141b18386ca7cbc5{color:red}`}</_JSXStyle>,
    className: "jsx-141b18386ca7cbc5"
};
const dynamic = (colors)=>{
    const b = {
        styles: <_JSXStyle id={"a6aaef75eecc989e"} dynamic={[
            colors.green.light
        ]}>{`div.__jsx-style-dynamic-selector{color:${colors.green.light}}a.__jsx-style-dynamic-selector{color:red}`}</_JSXStyle>,
        className: _JSXStyle.dynamic([
            [
                "a6aaef75eecc989e",
                [
                    colors.green.light
                ]
            ]
        ])
    };
};
export default {
    styles: <_JSXStyle id={"c1431f6a913dbbc9"}>{`div.jsx-c1431f6a913dbbc9{font-size:3em}p.jsx-c1431f6a913dbbc9{color:${color}}`}</_JSXStyle>,
    className: "jsx-c1431f6a913dbbc9"
};

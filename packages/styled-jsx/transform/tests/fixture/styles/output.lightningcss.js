import _JSXStyle from "styled-jsx/style";
import colors, { size } from './constants';
const color = 'red';
const bar = new String("div.jsx-aaed0341accea8f{font-size:3em}");
bar.__hash = "aaed0341accea8f";
const baz = new String("div{font-size:3em}");
baz.__hash = "aaed0341accea8f";
const a = new String(`div{font-size:${size}em}`);
a.__hash = "14b197463166f722";
export const uh = bar;
export const foo = new String(`div.jsx-f4f8570cd812466{color:${color}}`);
foo.__hash = "f4f8570cd812466";
({
    styles: <_JSXStyle id={"92e08135776a0568"}>{`div.jsx-92e08135776a0568{color:${colors.green.light}}a.jsx-92e08135776a0568{color:red}`}</_JSXStyle>,
    className: "jsx-92e08135776a0568"
});
const b = {
    styles: <_JSXStyle id={"92e08135776a0568"}>{`div.jsx-92e08135776a0568{color:${colors.green.light}}a.jsx-92e08135776a0568{color:red}`}</_JSXStyle>,
    className: "jsx-92e08135776a0568"
};
const dynamic = (colors)=>{
    const b = {
        styles: <_JSXStyle id={"30f6e55b71d9bcb1"} dynamic={[
            colors.green.light
        ]}>{`div.__jsx-style-dynamic-selector{color:${colors.green.light}}a.__jsx-style-dynamic-selector{color:red}`}</_JSXStyle>,
        className: _JSXStyle.dynamic([
            [
                "30f6e55b71d9bcb1",
                [
                    colors.green.light
                ]
            ]
        ])
    };
};
export default {
    styles: <_JSXStyle id={"5e2f288d77dfe5f7"}>{`div.jsx-5e2f288d77dfe5f7{font-size:3em}p.jsx-5e2f288d77dfe5f7{color:${color}}`}</_JSXStyle>,
    className: "jsx-5e2f288d77dfe5f7"
};

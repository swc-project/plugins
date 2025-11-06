import _JSXStyle from "styled-jsx/style";
import colors, { size } from "./constants";
const color = "red";
const bar = new String("div.jsx-aaed0341accea8f{font-size:3em}");
bar.__hash = "aaed0341accea8f";
const baz = new String("div{font-size:3em}");
baz.__hash = "aaed0341accea8f";
const a = new String(`div{font-size:${size}em}`);
a.__hash = "335ce71362ff6898";
export const uh = bar;
export const foo = new String(`div.jsx-770e2f89f694f0c4{color:${color}}`);
foo.__hash = "770e2f89f694f0c4";
({
    styles: <_JSXStyle id={"4bec2f6082dc66fb"}>{`div.jsx-4bec2f6082dc66fb{color:${colors.green.light}}a.jsx-4bec2f6082dc66fb{color:red}`}</_JSXStyle>,
    className: "jsx-4bec2f6082dc66fb"
});
const b = {
    styles: <_JSXStyle id={"36047a637012add5"}>{`div.jsx-36047a637012add5{color:${colors.green.light}}a.jsx-36047a637012add5{color:red}`}</_JSXStyle>,
    className: "jsx-36047a637012add5"
};
const dynamic = (colors)=>{
    const b = {
        styles: <_JSXStyle id={"7addc29b747c9d9f"} dynamic={[
            colors.green.light
        ]}>{`div.__jsx-style-dynamic-selector{color:${colors.green.light}}a.__jsx-style-dynamic-selector{color:red}`}</_JSXStyle>,
        className: _JSXStyle.dynamic([
            [
                "7addc29b747c9d9f",
                [
                    colors.green.light
                ]
            ]
        ])
    };
};
export default {
    styles: <_JSXStyle id={"c7a3824d3cef225d"}>{`div.jsx-c7a3824d3cef225d{font-size:3em}p.jsx-c7a3824d3cef225d{color:${color}}`}</_JSXStyle>,
    className: "jsx-c7a3824d3cef225d"
};

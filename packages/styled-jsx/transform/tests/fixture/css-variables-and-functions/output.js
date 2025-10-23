import _JSXStyle from "styled-jsx/style";
export default function Component() {
    const dynamicValue = '"dynamic content"';
    const color1 = '#FF0000';
    const color2 = '#0000FF';
    const offset = 5;
    const spacing = 10;
    return <div className={_JSXStyle.dynamic([
        [
            "906d88069a2f1c7e",
            [
                dynamicValue,
                color1,
                color2,
                offset,
                offset,
                spacing
            ]
        ]
    ]) + " " + "container"}>
      <div className={_JSXStyle.dynamic([
        [
            "906d88069a2f1c7e",
            [
                dynamicValue,
                color1,
                color2,
                offset,
                offset,
                spacing
            ]
        ]
    ]) + " " + "item"}>CSS Variables and Functions</div>
      <_JSXStyle id={"906d88069a2f1c7e"} dynamic={[
        dynamicValue,
        color1,
        color2,
        offset,
        offset,
        spacing
    ]}>{`.container.__jsx-style-dynamic-selector{--local-var:${dynamicValue};color:var(--text-color);background:linear-gradient(to right,${color1},${color2})}.container.__jsx-style-dynamic-selector .item.__jsx-style-dynamic-selector{transform:translate(calc(var(--x) + ${offset}px),calc(var(--y) + ${offset}px))}.container.__jsx-style-dynamic-selector div.__jsx-style-dynamic-selector{margin:calc(10px + ${spacing}px)}`}</_JSXStyle>
    </div>;
}

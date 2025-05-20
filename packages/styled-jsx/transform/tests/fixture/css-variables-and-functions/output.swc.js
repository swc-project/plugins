import _JSXStyle from "styled-jsx/style";
export default function Component() {
    const dynamicValue = '"dynamic content"';
    const color1 = '#FF0000';
    const color2 = '#0000FF';
    const offset = 5;
    const spacing = 10;
    return <div className={_JSXStyle.dynamic([
        [
            "ddaeead21092ae6d",
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
            "ddaeead21092ae6d",
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
      <_JSXStyle id={"ddaeead21092ae6d"} dynamic={[
        dynamicValue,
        color1,
        color2,
        offset,
        offset,
        spacing
    ]}>{`.container.__jsx-style-dynamic-selector{--local-var:${dynamicValue};color:var(--text-color);background:-webkit-linear-gradient(left,${color1},${color2});background:-moz-linear-gradient(left,${color1},${color2});background:-o-linear-gradient(left,${color1},${color2});background:linear-gradient(to right,${color1},${color2})}.container.__jsx-style-dynamic-selector .item.__jsx-style-dynamic-selector{-webkit-transform:translate(-webkit-calc(var(--x) + ${offset}px),-webkit-calc(var(--y) + ${offset}px));-moz-transform:translate(-moz-calc(var(--x) + ${offset}px),-moz-calc(var(--y) + ${offset}px));-ms-transform:translate(calc(var(--x) + ${offset}px),calc(var(--y) + ${offset}px));-o-transform:translate(calc(var(--x) + ${offset}px),calc(var(--y) + ${offset}px));transform:translate(-webkit-calc(var(--x) + ${offset}px),-webkit-calc(var(--y) + ${offset}px));transform:translate(-moz-calc(var(--x) + ${offset}px),-moz-calc(var(--y) + ${offset}px));transform:translate(calc(var(--x) + ${offset}px),calc(var(--y) + ${offset}px))}.container.__jsx-style-dynamic-selector div.__jsx-style-dynamic-selector{margin:-webkit-calc(10px + ${spacing}px);margin:-moz-calc(10px + ${spacing}px);margin:calc(10px + ${spacing}px)}`}</_JSXStyle>
    </div>;
}

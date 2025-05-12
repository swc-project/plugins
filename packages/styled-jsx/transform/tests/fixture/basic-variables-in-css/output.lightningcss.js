import _JSXStyle from "styled-jsx/style";
export default function Component() {
    const width = 100;
    const height = 200;
    const color = '#FF0000';
    return <div className={_JSXStyle.dynamic([
        [
            "fb6f0ccab88fe7c7",
            [
                width,
                height,
                color
            ]
        ]
    ])}>
      <h1 className={_JSXStyle.dynamic([
        [
            "fb6f0ccab88fe7c7",
            [
                width,
                height,
                color
            ]
        ]
    ])}>Basic Variables Test</h1>
      <_JSXStyle id={"fb6f0ccab88fe7c7"} dynamic={[
        width,
        height,
        color
    ]}>{`.component.__jsx-style-dynamic-selector{width:${width}px;height:${height}px;color:${color}}`}</_JSXStyle>
    </div>;
}

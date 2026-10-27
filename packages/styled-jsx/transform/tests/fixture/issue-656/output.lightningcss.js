import _JSXStyle from "styled-jsx/style";
export const Probe = (props)=>{
    const t = props.theme;
    const Icon = t.icon;
    return <div className={_JSXStyle.dynamic([
        [
            "9790cd69134f55ee",
            [
                t.color
            ]
        ]
    ]) + " " + "probe"}>
      <Icon className={_JSXStyle.dynamic([
        [
            "9790cd69134f55ee",
            [
                t.color
            ]
        ]
    ])}/>
      <_JSXStyle id={"9790cd69134f55ee"} dynamic={[
        t.color
    ]}>{`.probe.__jsx-style-dynamic-selector{color:${t.color}}`}</_JSXStyle>
    </div>;
};

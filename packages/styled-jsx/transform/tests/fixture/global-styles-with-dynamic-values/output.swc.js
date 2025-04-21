import _JSXStyle from "styled-jsx/style";
export default function Component() {
    const id = 'theme-1';
    const cssVariables = {
        '--primary-color': '#0070f3',
        '--secondary-color': '#ff0080'
    };
    const stringifyCssVariablesObject = (obj)=>{
        return Object.entries(obj).map(([key, value])=>`${key}: ${value};`).join('\n');
    };
    const buttonColor = 'var(--primary-color)';
    const backgroundColor = '#f0f0f0';
    return <div className={_JSXStyle.dynamic([
        [
            "8fabda4c069f3eb6",
            [
                id,
                stringifyCssVariablesObject(cssVariables),
                buttonColor,
                backgroundColor
            ]
        ]
    ]) + " " + `scope-${id}`}>
      <button className={_JSXStyle.dynamic([
        [
            "8fabda4c069f3eb6",
            [
                id,
                stringifyCssVariablesObject(cssVariables),
                buttonColor,
                backgroundColor
            ]
        ]
    ])}>Global Styled Button</button>
      <div className={_JSXStyle.dynamic([
        [
            "8fabda4c069f3eb6",
            [
                id,
                stringifyCssVariablesObject(cssVariables),
                buttonColor,
                backgroundColor
            ]
        ]
    ])}>Styled Div</div>
      <_JSXStyle id={"8fabda4c069f3eb6"} dynamic={[
        id,
        stringifyCssVariablesObject(cssVariables),
        buttonColor,
        backgroundColor
    ]}>{`.scope-${id}{${stringifyCssVariablesObject(cssVariables)} button { color: ${buttonColor}; }div { background-color: ${backgroundColor}; }}`}</_JSXStyle>
    </div>;
}

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
    return <div className={`scope-${id}`}>
      <button>Global Styled Button</button>
      <div>Styled Div</div>
      <_JSXStyle id={"global"} dynamic={[
        id,
        stringifyCssVariablesObject(cssVariables),
        buttonColor,
        backgroundColor
    ]}>{`.scope-${id}{${stringifyCssVariablesObject(cssVariables)}}.scope-${id} button{color:${buttonColor}}.scope-${id} div{background-color:${backgroundColor}}`}</_JSXStyle>
    </div>;
}
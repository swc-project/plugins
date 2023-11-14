import _JSXStyle from "styled-jsx/style";
const color = 'red';
const otherColor = 'green';
const A = ()=><div className={"jsx-b013e9f16b874595"}>

    <p className={"jsx-b013e9f16b874595"}>test</p>

    <_JSXStyle id={"b013e9f16b874595"}>{`p.jsx-b013e9f16b874595{color:${color}}`}</_JSXStyle>

  </div>;
const B = ()=><div className={"jsx-60dc7908034d9fdd"}>

    <p className={"jsx-60dc7908034d9fdd"}>test</p>

    <_JSXStyle id={"60dc7908034d9fdd"}>{`p.jsx-60dc7908034d9fdd{color:${otherColor}}`}</_JSXStyle>

  </div>;
export default (()=><div>

    <A/>

    <B/>

  </div>);

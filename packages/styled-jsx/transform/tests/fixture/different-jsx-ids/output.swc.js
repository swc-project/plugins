import _JSXStyle from "styled-jsx/style";
const color = "red";
const otherColor = "green";
const A = ()=><div className={"jsx-a2ac7ef155861a86"}>
    <p className={"jsx-a2ac7ef155861a86"}>test</p>
    <_JSXStyle id={"a2ac7ef155861a86"}>{`p.jsx-a2ac7ef155861a86{color:${color}}`}</_JSXStyle>
  </div>;
const B = ()=><div className={"jsx-9300b23ceb213812"}>
    <p className={"jsx-9300b23ceb213812"}>test</p>
    <_JSXStyle id={"9300b23ceb213812"}>{`p.jsx-9300b23ceb213812{color:${otherColor}}`}</_JSXStyle>
  </div>;
export default (()=><div>
    <A/>
    <B/>
  </div>);

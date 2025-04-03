import _JSXStyle from "styled-jsx/style";
const color = "red";
const otherColor = "green";
const A = ()=><div className={"jsx-8ec85d8cfc332192"}>
    <p className={"jsx-8ec85d8cfc332192"}>test</p>
    <_JSXStyle id={"8ec85d8cfc332192"}>{`p.jsx-8ec85d8cfc332192{color:${color}}`}</_JSXStyle>
  </div>;
const B = ()=><div className={"jsx-d31ff29832cfd736"}>
    <p className={"jsx-d31ff29832cfd736"}>test</p>
    <_JSXStyle id={"d31ff29832cfd736"}>{`p.jsx-d31ff29832cfd736{color:${otherColor}}`}</_JSXStyle>
  </div>;
export default (()=><div>
    <A/>
    <B/>
  </div>);

import _JSXStyle from "styled-jsx/style";
const color = "red";
const otherColor = "green";
const A = ()=><div className={"jsx-806720d41ae9fbf8"}>
    <p className={"jsx-806720d41ae9fbf8"}>test</p>
    <_JSXStyle id={"806720d41ae9fbf8"}>{`p.jsx-806720d41ae9fbf8{color:${color}}`}</_JSXStyle>
  </div>;
const B = ()=><div className={"jsx-5a7e7d24349275f2"}>
    <p className={"jsx-5a7e7d24349275f2"}>test</p>
    <_JSXStyle id={"5a7e7d24349275f2"}>{`p.jsx-5a7e7d24349275f2{color:${otherColor}}`}</_JSXStyle>
  </div>;
export default (()=><div>
    <A/>
    <B/>
  </div>);

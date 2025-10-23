import _JSXStyle from "styled-jsx/style";
const color = "red";
const otherColor = "green";
const A = ()=><div className={"jsx-6b8b91efa51734b5"}>
    <p className={"jsx-6b8b91efa51734b5"}>test</p>
    <_JSXStyle id={"6b8b91efa51734b5"}>{`p.jsx-6b8b91efa51734b5{color:${color}}`}</_JSXStyle>
  </div>;
const B = ()=><div className={"jsx-366f6aa79b958630"}>
    <p className={"jsx-366f6aa79b958630"}>test</p>
    <_JSXStyle id={"366f6aa79b958630"}>{`p.jsx-366f6aa79b958630{color:${otherColor}}`}</_JSXStyle>
  </div>;
export default (()=><div>
    <A/>
    <B/>
  </div>);

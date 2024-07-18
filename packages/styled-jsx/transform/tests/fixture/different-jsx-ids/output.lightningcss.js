import _JSXStyle from "styled-jsx/style";
const color = "red";
const otherColor = "green";
const A = ()=><div className={"jsx-ba5593a7ec9cac44"}>
    <p className={"jsx-ba5593a7ec9cac44"}>test</p>
    <_JSXStyle id={"ba5593a7ec9cac44"}>{`p.jsx-ba5593a7ec9cac44{color:${color}}`}</_JSXStyle>
  </div>;
const B = ()=><div className={"jsx-ddf7b1ec0b39ee5b"}>
    <p className={"jsx-ddf7b1ec0b39ee5b"}>test</p>
    <_JSXStyle id={"ddf7b1ec0b39ee5b"}>{`p.jsx-ddf7b1ec0b39ee5b{color:${otherColor}}`}</_JSXStyle>
  </div>;
export default (()=><div>
    <A/>
    <B/>
  </div>);

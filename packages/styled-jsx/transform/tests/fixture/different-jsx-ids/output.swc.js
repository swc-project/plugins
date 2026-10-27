import _JSXStyle from "styled-jsx/style";
const color = "red";
const otherColor = "green";
const A = ()=><div className={"jsx-b940b8041b63a0fa"}>
    <p className={"jsx-b940b8041b63a0fa"}>test</p>
    <_JSXStyle id={"b940b8041b63a0fa"}>{`p.jsx-b940b8041b63a0fa{color:${color}}`}</_JSXStyle>
  </div>;
const B = ()=><div className={"jsx-664aac2d10a660c"}>
    <p className={"jsx-664aac2d10a660c"}>test</p>
    <_JSXStyle id={"664aac2d10a660c"}>{`p.jsx-664aac2d10a660c{color:${otherColor}}`}</_JSXStyle>
  </div>;
export default (()=><div>
    <A/>
    <B/>
  </div>);

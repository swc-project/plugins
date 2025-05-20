import _JSXStyle from "styled-jsx/style";
const color = "red";
const otherColor = "green";
const A = ()=><div className={"jsx-3da79906f53778b8"}>
    <p className={"jsx-3da79906f53778b8"}>test</p>
    <_JSXStyle id={"3da79906f53778b8"}>{`p.jsx-3da79906f53778b8{color:${color}}`}</_JSXStyle>
  </div>;
const B = ()=><div className={"jsx-191e14568be8ee08"}>
    <p className={"jsx-191e14568be8ee08"}>test</p>
    <_JSXStyle id={"191e14568be8ee08"}>{`p.jsx-191e14568be8ee08{color:${otherColor}}`}</_JSXStyle>
  </div>;
export default (()=><div>
    <A/>
    <B/>
  </div>);

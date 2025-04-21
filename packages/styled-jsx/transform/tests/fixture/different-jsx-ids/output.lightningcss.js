import _JSXStyle from "styled-jsx/style";
const color = "red";
const otherColor = "green";
const A = ()=><div className={"jsx-9f19e858015e072e"}>
    <p className={"jsx-9f19e858015e072e"}>test</p>
    <_JSXStyle id={"9f19e858015e072e"}>{`p.jsx-9f19e858015e072e{color:${color}}`}</_JSXStyle>
  </div>;
const B = ()=><div className={"jsx-5f7504ea15d7d51e"}>
    <p className={"jsx-5f7504ea15d7d51e"}>test</p>
    <_JSXStyle id={"5f7504ea15d7d51e"}>{`p.jsx-5f7504ea15d7d51e{color:${otherColor}}`}</_JSXStyle>
  </div>;
export default (()=><div>
    <A/>
    <B/>
  </div>);

import _JSXStyle from "styled-jsx/style";
const color = "red";
const otherColor = "green";
const A = ()=><div className={"jsx-9184bb6edf7bc64f"}>

    <p className={"jsx-9184bb6edf7bc64f"}>test</p>

    <_JSXStyle id={"9184bb6edf7bc64f"}>{`p.jsx-9184bb6edf7bc64f{color:${color}}`}</_JSXStyle>

  </div>;
const B = ()=><div className={"jsx-ff208b3ac3acec91"}>

    <p className={"jsx-ff208b3ac3acec91"}>test</p>

    <_JSXStyle id={"ff208b3ac3acec91"}>{`p.jsx-ff208b3ac3acec91{color:${otherColor}}`}</_JSXStyle>

  </div>;
export default (()=><div>

    <A/>

    <B/>

  </div>);

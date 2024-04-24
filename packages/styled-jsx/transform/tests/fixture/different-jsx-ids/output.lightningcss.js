import _JSXStyle from "styled-jsx/style";
const color = "red";
const otherColor = "green";
const A = ()=><div className={"jsx-7148bb7ee4fbe10d"}>

    <p className={"jsx-7148bb7ee4fbe10d"}>test</p>

    <_JSXStyle id={"7148bb7ee4fbe10d"}>{`p.jsx-7148bb7ee4fbe10d{color:${color}}`}</_JSXStyle>

  </div>;
const B = ()=><div className={"jsx-702a462674d88be"}>

    <p className={"jsx-702a462674d88be"}>test</p>

    <_JSXStyle id={"702a462674d88be"}>{`p.jsx-702a462674d88be{color:${otherColor}}`}</_JSXStyle>

  </div>;
export default (()=><div>

    <A/>

    <B/>

  </div>);

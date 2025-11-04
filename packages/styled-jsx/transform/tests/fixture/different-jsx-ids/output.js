import _JSXStyle from "styled-jsx/style";
const color = "red";
const otherColor = "green";
const A = ()=><div className={"jsx-c97f2e17d45fcb22"}>
    <p className={"jsx-c97f2e17d45fcb22"}>test</p>
    <_JSXStyle id={"c97f2e17d45fcb22"}>{`p.jsx-c97f2e17d45fcb22{color:${color}}`}</_JSXStyle>
  </div>;
const B = ()=><div className={"jsx-baa9af07b36dbed2"}>
    <p className={"jsx-baa9af07b36dbed2"}>test</p>
    <_JSXStyle id={"baa9af07b36dbed2"}>{`p.jsx-baa9af07b36dbed2{color:${otherColor}}`}</_JSXStyle>
  </div>;
export default (()=><div>
    <A/>
    <B/>
  </div>);

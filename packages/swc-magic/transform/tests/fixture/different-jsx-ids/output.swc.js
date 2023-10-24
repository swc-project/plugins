import _JSXStyle from "swc-magic/style";
const color = 'red';
const otherColor = 'green';
const A = ()=><div className={"jsx-3f087c835a50190d"}>

    <p className={"jsx-3f087c835a50190d"}>test</p>

    <_JSXStyle id={"3f087c835a50190d"}>{`p.jsx-3f087c835a50190d{color:${color}}`}</_JSXStyle>

  </div>;
const B = ()=><div className={"jsx-d051a1c8140076ed"}>

    <p className={"jsx-d051a1c8140076ed"}>test</p>

    <_JSXStyle id={"d051a1c8140076ed"}>{`p.jsx-d051a1c8140076ed{color:${otherColor}}`}</_JSXStyle>

  </div>;
export default (()=><div>

    <A/>

    <B/>

  </div>);

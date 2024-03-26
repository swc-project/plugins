import _JSXStyle from "styled-jsx/style";
export default function Home({ fontFamily }) {
  return (
    <div className={_JSXStyle.dynamic([["c8e2e31fc7e7bb57", [fontFamily]]])}>
      <_JSXStyle
        id={"c8e2e31fc7e7bb57"}
        dynamic={[fontFamily]}
      >{`body{font-family:${fontFamily}}code:before,code:after{content:"\`"}`}</_JSXStyle>
    </div>
  );
}

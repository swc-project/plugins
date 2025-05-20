import _JSXStyle from "styled-jsx/style";
export default function Home() {
    const breakpoint = "500px";
    return <div className={"jsx-6b06eb4a865c12db"}>
      <div className={"jsx-6b06eb4a865c12db" + " " + "container"}>
        container (should be blue)
        <div className={"jsx-6b06eb4a865c12db" + " " + "responsive"}>
          responsive (purple on mobile, orange on desktop)
        </div>
      </div>

      <_JSXStyle id={"6b06eb4a865c12db"}>{`.container.jsx-6b06eb4a865c12db{color:#00f;padding:3rem}@media (width<=${breakpoint}){.container.jsx-6b06eb4a865c12db .responsive.jsx-6b06eb4a865c12db{color:purple}}`}</_JSXStyle>
    </div>;
}

import _JSXStyle from "styled-jsx/style";
export default function Home() {
    const breakpoint = "500px";
    return <div className={"jsx-3f85910312d1cf7b"}>
      <div className={"jsx-3f85910312d1cf7b" + " " + "container"}>
        container (should be blue)
        <div className={"jsx-3f85910312d1cf7b" + " " + "responsive"}>
          responsive (purple on mobile, orange on desktop)
        </div>
      </div>

      <_JSXStyle id={"3f85910312d1cf7b"}>{`.container.jsx-3f85910312d1cf7b{color:#00f;padding:3rem}@media (width<=${breakpoint}){.container.jsx-3f85910312d1cf7b .responsive.jsx-3f85910312d1cf7b{color:purple}}`}</_JSXStyle>
    </div>;
}

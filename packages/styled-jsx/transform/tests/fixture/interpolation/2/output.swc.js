import _JSXStyle from "styled-jsx/style";
export default function Home() {
    const breakpoint = "500px";
    return <div className={"jsx-f3f9c604d7103d8c"}>
      <div className={"jsx-f3f9c604d7103d8c" + " " + "container"}>
        container (should be blue)
        <div className={"jsx-f3f9c604d7103d8c" + " " + "responsive"}>
          responsive (purple on mobile, orange on desktop)
        </div>
      </div>

      <_JSXStyle id={"f3f9c604d7103d8c"}>{`.container.jsx-f3f9c604d7103d8c{color:#00f;padding:3rem}@media (width<=${breakpoint}){.container.jsx-f3f9c604d7103d8c .responsive.jsx-f3f9c604d7103d8c{color:purple}}`}</_JSXStyle>
    </div>;
}

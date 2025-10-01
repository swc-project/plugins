import _JSXStyle from "styled-jsx/style";
export default function Home() {
    const breakpoint = "500px";
    return <div className={"jsx-aa91a2d0605d400e"}>
      <div className={"jsx-aa91a2d0605d400e" + " " + "container"}>
        container (should be blue)
        <div className={"jsx-aa91a2d0605d400e" + " " + "responsive"}>
          responsive (purple on mobile, orange on desktop)
        </div>
      </div>

      <_JSXStyle id={"aa91a2d0605d400e"}>{`.container.jsx-aa91a2d0605d400e{color:#00f;padding:3rem}@media (max-width:${breakpoint}){.container.jsx-aa91a2d0605d400e .responsive.jsx-aa91a2d0605d400e{color:purple}}`}</_JSXStyle>
    </div>;
}

import _JSXStyle from "styled-jsx/style";
export default function Home() {
    const breakpoint = "500px";
    return <div className={"jsx-81df6e2eb900c146"}>
      <div className={"jsx-81df6e2eb900c146" + " " + "container"}>
        container (should be blue)
        <div className={"jsx-81df6e2eb900c146" + " " + "responsive"}>
          responsive (purple on mobile, orange on desktop)
        </div>
      </div>

      <_JSXStyle id={"81df6e2eb900c146"}>{`.container.jsx-81df6e2eb900c146{color:#00f;padding:3rem}@media (max-width:${breakpoint}){.container.jsx-81df6e2eb900c146 .responsive.jsx-81df6e2eb900c146{color:purple}}`}</_JSXStyle>
    </div>;
}

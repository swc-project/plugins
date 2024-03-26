import _JSXStyle from "styled-jsx/style";
export default class {
  render() {
    return (
      <div
        className={_JSXStyle.dynamic([
          ["c683c04b93f59aa9", [a || "var(--c)", b || "inherit"]],
        ])}
      >
        <p
          className={_JSXStyle.dynamic([
            ["c683c04b93f59aa9", [a || "var(--c)", b || "inherit"]],
          ])}
        >
          test
        </p>

        <_JSXStyle
          id={"c683c04b93f59aa9"}
          dynamic={[a || "var(--c)", b || "inherit"]}
        >{`.a:hover .b.__jsx-style-dynamic-selector{display:inline-block;padding:0 ${a || "var(--c)"};color:${b || "inherit"}}`}</_JSXStyle>
      </div>
    );
  }
}

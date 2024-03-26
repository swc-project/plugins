import _JSXStyle from "styled-jsx/style";
function NavigationItem({ active, className }) {
  return (
    <span
      className={
        "jsx-e73bf96b356a8634" +
        " " +
        (cn(
          {
            active,
          },
          className,
          "navigation-item",
        ) || "")
      }
    >
      <_JSXStyle id={"e73bf96b356a8634"}>
        {
          '.navigation-item.jsx-e73bf96b356a8634 a:after{content:attr(data-text);content:attr(data-text)/""}'
        }
      </_JSXStyle>
    </span>
  );
}
export default NavigationItem;

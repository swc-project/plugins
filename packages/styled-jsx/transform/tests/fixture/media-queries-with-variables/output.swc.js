import _JSXStyle from "styled-jsx/style";
export default function Component() {
    const ResponsiveBreakpoint = {
        mobile: '768px',
        tablet: '1024px',
        desktop: '1440px'
    };
    const breakpoint = 'mobile';
    const mobileWidth = 320;
    return <div className={_JSXStyle.dynamic([
        [
            "a4b3e2f1d0c9b8a7",
            [
                ResponsiveBreakpoint[breakpoint],
                mobileWidth
            ]
        ]
    ]) + " component"}>
      <div className={_JSXStyle.dynamic([
        [
            "a4b3e2f1d0c9b8a7",
            [
                ResponsiveBreakpoint[breakpoint],
                mobileWidth
            ]
        ]
    ]) + " active"}>Responsive Element</div>
      <_JSXStyle id={"a4b3e2f1d0c9b8a7"} dynamic={[
        ResponsiveBreakpoint[breakpoint],
        mobileWidth
    ]}>{`.component.__jsx-style-dynamic-selector{width:100%}@media (max-width:${ResponsiveBreakpoint[breakpoint]}){.component.__jsx-style-dynamic-selector{width:${mobileWidth}px}.component.__jsx-style-dynamic-selector.active{color:blue}.component.__jsx-style-dynamic-selector div{display:block}}`}</_JSXStyle>
    </div>;
}
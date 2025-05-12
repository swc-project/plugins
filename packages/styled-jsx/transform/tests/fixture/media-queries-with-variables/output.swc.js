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
            "3ee1174a56442a12",
            [
                ResponsiveBreakpoint[breakpoint],
                mobileWidth
            ]
        ]
    ]) + " " + "component"}>
      <div className={_JSXStyle.dynamic([
        [
            "3ee1174a56442a12",
            [
                ResponsiveBreakpoint[breakpoint],
                mobileWidth
            ]
        ]
    ]) + " " + "active"}>Responsive Element</div>
      <_JSXStyle id={"3ee1174a56442a12"} dynamic={[
        ResponsiveBreakpoint[breakpoint],
        mobileWidth
    ]}>{`.component.__jsx-style-dynamic-selector{width:100%}@media (width<=${ResponsiveBreakpoint[breakpoint]}){.component.__jsx-style-dynamic-selector{width:${mobileWidth}px}.component.__jsx-style-dynamic-selector.active.__jsx-style-dynamic-selector{color:#00f}.component.__jsx-style-dynamic-selector div.__jsx-style-dynamic-selector{display:block}}`}</_JSXStyle>
    </div>;
}

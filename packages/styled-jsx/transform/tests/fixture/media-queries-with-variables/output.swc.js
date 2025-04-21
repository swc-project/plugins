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
            "f27916106b885486",
            [
                ResponsiveBreakpoint[breakpoint],
                mobileWidth
            ]
        ]
    ]) + " " + "component"}>
      <div className={_JSXStyle.dynamic([
        [
            "f27916106b885486",
            [
                ResponsiveBreakpoint[breakpoint],
                mobileWidth
            ]
        ]
    ]) + " " + "active"}>Responsive Element</div>
      <_JSXStyle id={"f27916106b885486"} dynamic={[
        ResponsiveBreakpoint[breakpoint],
        mobileWidth
    ]}>{`.component.__jsx-style-dynamic-selector{width:100%}`}</_JSXStyle>
    </div>;
}

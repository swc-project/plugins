import _JSXStyle from "swc-magic/style";
export default class {
    render() {
        return <div className={_JSXStyle.dynamic([
            [
                "293094c41b76e3ed",
                [
                    a || 'var(--c)',
                    b || 'inherit'
                ]
            ]
        ])}>

          <p className={_JSXStyle.dynamic([
            [
                "293094c41b76e3ed",
                [
                    a || 'var(--c)',
                    b || 'inherit'
                ]
            ]
        ])}>test</p>

          <_JSXStyle id={"293094c41b76e3ed"} dynamic={[
            a || 'var(--c)',
            b || 'inherit'
        ]}>{`.a:hover .b.__jsx-style-dynamic-selector{display:inline-block;padding:0 ${a || 'var(--c)'};color:${b || 'inherit'}}`}</_JSXStyle>

        </div>;
    }
}

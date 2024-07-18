import _JSXStyle from "styled-jsx/style";
export default class {
    render() {
        return <div className={_JSXStyle.dynamic([
            [
                "8a9c5d01fc6fe64e",
                [
                    a ? "100%" : "200px",
                    b ? "0" : "8px 20px"
                ]
            ]
        ])}>
        <p className={_JSXStyle.dynamic([
            [
                "8a9c5d01fc6fe64e",
                [
                    a ? "100%" : "200px",
                    b ? "0" : "8px 20px"
                ]
            ]
        ])}>test</p>
        <_JSXStyle id={"8a9c5d01fc6fe64e"} dynamic={[
            a ? "100%" : "200px",
            b ? "0" : "8px 20px"
        ]}>{`.item.__jsx-style-dynamic-selector{max-width:${a ? "100%" : "200px"};padding:${b ? "0" : "8px 20px"}}`}</_JSXStyle>
      </div>;
    }
}

import _JSXStyle from "styled-jsx/style";
export default class {
    render() {
        return <div className={_JSXStyle.dynamic([
            [
                "c6ab765fd85455ec",
                [
                    inputSize ? "height: calc(2 * var(--a)) !important;" : ""
                ]
            ]
        ])}>

        <p className={_JSXStyle.dynamic([
            [
                "c6ab765fd85455ec",
                [
                    inputSize ? "height: calc(2 * var(--a)) !important;" : ""
                ]
            ]
        ])}>test</p>

        <_JSXStyle id={"c6ab765fd85455ec"} dynamic={[
            inputSize ? "height: calc(2 * var(--a)) !important;" : ""
        ]}>{`@media only screen{a.__jsx-style-dynamic-selector{${inputSize ? "height: calc(2 * var(--a)) !important;" : ""}}}`}</_JSXStyle>

      </div>;
    }
}

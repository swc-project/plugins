import _JSXStyle from "styled-jsx/style";
const CUTOUT_AVATAR_PERCENTAGE_VISIBLE = Math.random();
const HEAD_MARGIN_PERCENTAGE = Math.random();
const MaskedDivBad = ()=>{
    return <>
            <div className={_JSXStyle.dynamic([
        [
            "59987c08c9f7d258",
            [
                0.5 + HEAD_MARGIN_PERCENTAGE,
                0.5 + CUTOUT_AVATAR_PERCENTAGE_VISIBLE + HEAD_MARGIN_PERCENTAGE
            ]
        ]
    ]) + " " + "head"}>
                <div className={_JSXStyle.dynamic([
        [
            "59987c08c9f7d258",
            [
                0.5 + HEAD_MARGIN_PERCENTAGE,
                0.5 + CUTOUT_AVATAR_PERCENTAGE_VISIBLE + HEAD_MARGIN_PERCENTAGE
            ]
        ]
    ]) + " " + "avatar-wrapper"}/>
            </div>
            <_JSXStyle id={"59987c08c9f7d258"} dynamic={[
        0.5 + HEAD_MARGIN_PERCENTAGE,
        0.5 + CUTOUT_AVATAR_PERCENTAGE_VISIBLE + HEAD_MARGIN_PERCENTAGE
    ]}>{`.head.__jsx-style-dynamic-selector{position:relative}.avatar-wrapper.__jsx-style-dynamic-selector{-webkit-mask-composite:source-out;background:#ff6b6b;border-radius:50%;width:40px;height:40px;-webkit-mask-image:url("data:image/svg+xml,<svg xmlns=\\"http://www.w3.org/2000/svg\\" viewBox=\\"0 0 1 1\\"><circle r=\\"0.5\\" cx=\\"0.5\\" cy=\\"0.5\\"/></svg>"),url("data:image/svg+xml,<svg xmlns=\\"http://www.w3.org/2000/svg\\" viewBox=\\"0 0 1 1\\"><circle r=\\"${0.5 + HEAD_MARGIN_PERCENTAGE}\\" cx=\\"${0.5 + CUTOUT_AVATAR_PERCENTAGE_VISIBLE + HEAD_MARGIN_PERCENTAGE}\\" cy=\\"0.5\\"/></svg>");mask-image:url("data:image/svg+xml,<svg xmlns=\\"http://www.w3.org/2000/svg\\" viewBox=\\"0 0 1 1\\"><circle r=\\"0.5\\" cx=\\"0.5\\" cy=\\"0.5\\"/></svg>"),url("data:image/svg+xml,<svg xmlns=\\"http://www.w3.org/2000/svg\\" viewBox=\\"0 0 1 1\\"><circle r=\\"${0.5 + HEAD_MARGIN_PERCENTAGE}\\" cx=\\"${0.5 + CUTOUT_AVATAR_PERCENTAGE_VISIBLE + HEAD_MARGIN_PERCENTAGE}\\" cy=\\"0.5\\"/></svg>");-webkit-mask-position:50%;mask-position:50%;-webkit-mask-size:100% 100%;mask-size:100% 100%;-webkit-mask-repeat:no-repeat;mask-repeat:no-repeat;-webkit-mask-composite:source-out;mask-composite:subtract}`}</_JSXStyle>
        </>;
};

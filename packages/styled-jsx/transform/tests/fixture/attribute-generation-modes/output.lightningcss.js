import _JSXStyle from "styled-jsx/style";
import styles from "./styles";
const styles2 = require("./styles2");
// external only
export const Test1 = ()=><div className={`jsx-${styles2.__hash} jsx-${styles.__hash}`}>
    <p className={`jsx-${styles2.__hash} jsx-${styles.__hash}`}>external only</p>
    <_JSXStyle id={styles.__hash}>{styles}</_JSXStyle>
    <_JSXStyle id={styles2.__hash}>{styles2}</_JSXStyle>
  </div>;
// external and static
export const Test2 = ()=><div className={"jsx-81a68341e430a972 " + `jsx-${styles.__hash}`}>
    <p className={"jsx-81a68341e430a972 " + `jsx-${styles.__hash}`}>external and static</p>
    <_JSXStyle id={"81a68341e430a972"}>{"p.jsx-81a68341e430a972{color:red}"}</_JSXStyle>
    <_JSXStyle id={styles.__hash}>{styles}</_JSXStyle>
  </div>;
// external and dynamic
export const Test3 = ({ color })=><div className={`jsx-${styles.__hash}` + " " + _JSXStyle.dynamic([
        [
            "7113a62c00624bef",
            [
                color
            ]
        ]
    ])}>
    <p className={`jsx-${styles.__hash}` + " " + _JSXStyle.dynamic([
        [
            "7113a62c00624bef",
            [
                color
            ]
        ]
    ])}>external and dynamic</p>
    <_JSXStyle id={"7113a62c00624bef"} dynamic={[
        color
    ]}>{`p.__jsx-style-dynamic-selector{color:${color}}`}</_JSXStyle>
    <_JSXStyle id={styles.__hash}>{styles}</_JSXStyle>
  </div>;
// external, static and dynamic
export const Test4 = ({ color })=><div className={`jsx-${styles.__hash}` + " jsx-ceba8c9ce34e3d0c " + _JSXStyle.dynamic([
        [
            "8db39ad50e58394a",
            [
                color
            ]
        ]
    ])}>
    <p className={`jsx-${styles.__hash}` + " jsx-ceba8c9ce34e3d0c " + _JSXStyle.dynamic([
        [
            "8db39ad50e58394a",
            [
                color
            ]
        ]
    ])}>external, static and dynamic</p>
    <_JSXStyle id={"ceba8c9ce34e3d0c"}>{"p.jsx-ceba8c9ce34e3d0c{display:inline-block}"}</_JSXStyle>
    <_JSXStyle id={"8db39ad50e58394a"} dynamic={[
        color
    ]}>{`p.__jsx-style-dynamic-selector{color:${color}}`}</_JSXStyle>
    <_JSXStyle id={styles.__hash}>{styles}</_JSXStyle>
  </div>;
// static only
export const Test5 = ()=><div className={"jsx-df0159ebd3f9fb6f"}>
    <p className={"jsx-df0159ebd3f9fb6f"}>static only</p>
    <_JSXStyle id={"ceba8c9ce34e3d0c"}>{"p.jsx-df0159ebd3f9fb6f{display:inline-block}"}</_JSXStyle>
    <_JSXStyle id={"81a68341e430a972"}>{"p.jsx-df0159ebd3f9fb6f{color:red}"}</_JSXStyle>
  </div>;
// static and dynamic
export const Test6 = ({ color })=><div className={"jsx-ceba8c9ce34e3d0c " + _JSXStyle.dynamic([
        [
            "4375826949b9259f",
            [
                color
            ]
        ]
    ])}>
    <p className={"jsx-ceba8c9ce34e3d0c " + _JSXStyle.dynamic([
        [
            "4375826949b9259f",
            [
                color
            ]
        ]
    ])}>static and dynamic</p>
    <_JSXStyle id={"ceba8c9ce34e3d0c"}>{"p.jsx-ceba8c9ce34e3d0c{display:inline-block}"}</_JSXStyle>
    <_JSXStyle id={"4375826949b9259f"} dynamic={[
        color
    ]}>{`p.__jsx-style-dynamic-selector{color:${color}}`}</_JSXStyle>
  </div>;
// dynamic only
export const Test7 = ({ color })=><div className={_JSXStyle.dynamic([
        [
            "c6c05a90f95d6b50",
            [
                color
            ]
        ]
    ])}>
    <p className={_JSXStyle.dynamic([
        [
            "c6c05a90f95d6b50",
            [
                color
            ]
        ]
    ])}>dynamic only</p>
    <_JSXStyle id={"c6c05a90f95d6b50"} dynamic={[
        color
    ]}>{`p.__jsx-style-dynamic-selector{color:${color}}`}</_JSXStyle>
  </div>;
// dynamic with scoped compound variable
export const Test8 = ({ color })=>{
    if (color) {
        const innerProps = {
            color
        };
        return <div className={_JSXStyle.dynamic([
            [
                "3a5cdfa6d75c82e1",
                [
                    innerProps.color
                ]
            ]
        ])}>
        <p className={_JSXStyle.dynamic([
            [
                "3a5cdfa6d75c82e1",
                [
                    innerProps.color
                ]
            ]
        ])}>dynamic with scoped compound variable</p>
        <_JSXStyle id={"3a5cdfa6d75c82e1"} dynamic={[
            innerProps.color
        ]}>{`p.__jsx-style-dynamic-selector{color:${innerProps.color}}`}</_JSXStyle>
      </div>;
    }
};
// dynamic with compound variable
export const Test9 = ({ color })=>{
    const innerProps = {
        color
    };
    return <div className={_JSXStyle.dynamic([
        [
            "78eaf0c7b6cf1e02",
            [
                innerProps.color
            ]
        ]
    ])}>
      <p className={_JSXStyle.dynamic([
        [
            "78eaf0c7b6cf1e02",
            [
                innerProps.color
            ]
        ]
    ])}>dynamic with compound variable</p>
      <_JSXStyle id={"78eaf0c7b6cf1e02"} dynamic={[
        innerProps.color
    ]}>{`p.__jsx-style-dynamic-selector{color:${innerProps.color}}`}</_JSXStyle>
    </div>;
};
const foo = "red";
// dynamic with constant variable
export const Test10 = ()=><div className={"jsx-568a62763d5e1d43"}>
    <p className={"jsx-568a62763d5e1d43"}>dynamic with constant variable</p>
    <_JSXStyle id={"568a62763d5e1d43"}>{`p.jsx-568a62763d5e1d43{color:${foo}}`}</_JSXStyle>
  </div>;
// dynamic with complex scope
export const Test11 = ({ color })=>{
    const items = Array.from({
        length: 5
    }).map((item, i)=><li key={i} className={_JSXStyle.dynamic([
            [
                "6074b9c2542c8080",
                [
                    color
                ]
            ]
        ]) + " " + "item"}>
      <_JSXStyle id={"6074b9c2542c8080"} dynamic={[
            color
        ]}>{`.item.__jsx-style-dynamic-selector{color:${color}}`}</_JSXStyle>
      Item #{i + 1}
    </li>);
    return <ul className="items">{items}</ul>;
};

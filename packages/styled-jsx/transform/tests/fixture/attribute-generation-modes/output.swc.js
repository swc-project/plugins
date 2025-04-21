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
            "9cf0861e627eb460",
            [
                color
            ]
        ]
    ])}>
    <p className={`jsx-${styles.__hash}` + " " + _JSXStyle.dynamic([
        [
            "9cf0861e627eb460",
            [
                color
            ]
        ]
    ])}>external and dynamic</p>
    <_JSXStyle id={"9cf0861e627eb460"} dynamic={[
        color
    ]}>{`p.__jsx-style-dynamic-selector{color:${color}}`}</_JSXStyle>
    <_JSXStyle id={styles.__hash}>{styles}</_JSXStyle>
  </div>;
// external, static and dynamic
export const Test4 = ({ color })=><div className={`jsx-${styles.__hash}` + " jsx-ceba8c9ce34e3d0c " + _JSXStyle.dynamic([
        [
            "a6db73ea1614bd2a",
            [
                color
            ]
        ]
    ])}>
    <p className={`jsx-${styles.__hash}` + " jsx-ceba8c9ce34e3d0c " + _JSXStyle.dynamic([
        [
            "a6db73ea1614bd2a",
            [
                color
            ]
        ]
    ])}>external, static and dynamic</p>
    <_JSXStyle id={"ceba8c9ce34e3d0c"}>{"p.jsx-ceba8c9ce34e3d0c{display:inline-block}"}</_JSXStyle>
    <_JSXStyle id={"a6db73ea1614bd2a"} dynamic={[
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
            "a193d505d3d29290",
            [
                color
            ]
        ]
    ])}>
    <p className={"jsx-ceba8c9ce34e3d0c " + _JSXStyle.dynamic([
        [
            "a193d505d3d29290",
            [
                color
            ]
        ]
    ])}>static and dynamic</p>
    <_JSXStyle id={"ceba8c9ce34e3d0c"}>{"p.jsx-ceba8c9ce34e3d0c{display:inline-block}"}</_JSXStyle>
    <_JSXStyle id={"a193d505d3d29290"} dynamic={[
        color
    ]}>{`p.__jsx-style-dynamic-selector{color:${color}}`}</_JSXStyle>
  </div>;
// dynamic only
export const Test7 = ({ color })=><div className={_JSXStyle.dynamic([
        [
            "82909201ec6d5927",
            [
                color
            ]
        ]
    ])}>
    <p className={_JSXStyle.dynamic([
        [
            "82909201ec6d5927",
            [
                color
            ]
        ]
    ])}>dynamic only</p>
    <_JSXStyle id={"82909201ec6d5927"} dynamic={[
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
                "309d50d3d6ddbbb7",
                [
                    innerProps.color
                ]
            ]
        ])}>
        <p className={_JSXStyle.dynamic([
            [
                "309d50d3d6ddbbb7",
                [
                    innerProps.color
                ]
            ]
        ])}>dynamic with scoped compound variable</p>
        <_JSXStyle id={"309d50d3d6ddbbb7"} dynamic={[
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
            "5f2ac2277e48cde2",
            [
                innerProps.color
            ]
        ]
    ])}>
      <p className={_JSXStyle.dynamic([
        [
            "5f2ac2277e48cde2",
            [
                innerProps.color
            ]
        ]
    ])}>dynamic with compound variable</p>
      <_JSXStyle id={"5f2ac2277e48cde2"} dynamic={[
        innerProps.color
    ]}>{`p.__jsx-style-dynamic-selector{color:${innerProps.color}}`}</_JSXStyle>
    </div>;
};
const foo = "red";
// dynamic with constant variable
export const Test10 = ()=><div className={"jsx-e8ee30f1b62250d5"}>
    <p className={"jsx-e8ee30f1b62250d5"}>dynamic with constant variable</p>
    <_JSXStyle id={"e8ee30f1b62250d5"}>{`p.jsx-e8ee30f1b62250d5{color:${foo}}`}</_JSXStyle>
  </div>;
// dynamic with complex scope
export const Test11 = ({ color })=>{
    const items = Array.from({
        length: 5
    }).map((item, i)=><li key={i} className={_JSXStyle.dynamic([
            [
                "dc477f2727c2a1d2",
                [
                    color
                ]
            ]
        ]) + " " + "item"}>
      <_JSXStyle id={"dc477f2727c2a1d2"} dynamic={[
            color
        ]}>{`.item.__jsx-style-dynamic-selector{color:${color}}`}</_JSXStyle>
      Item #{i + 1}
    </li>);
    return <ul className="items">{items}</ul>;
};

import _JSXStyle from "styled-jsx/style";
import React from "react";
// This should error with https://nextjs.org/docs/messages/nested-styled-jsx-tags
// but does not.
export default function SimplePage() {
    return <ComponentWithChildAsProp trigger={<>
                    <div className={"jsx-68234eda9c798fae" + " " + "text animated"}>Text</div>

                    <_JSXStyle id={"68234eda9c798fae"}>{".text.jsx-68234eda9c798fae{color:#00f}.text.jsx-68234eda9c798fae:hover{color:red}"}</_JSXStyle>
                </>}/>;
}
const ComponentWithChildAsProp = ({ trigger })=>{
    return <div>{trigger}</div>;
};

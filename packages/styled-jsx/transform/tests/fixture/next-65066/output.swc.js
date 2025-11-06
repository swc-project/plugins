import _JSXStyle from "styled-jsx/style";
import React from "react";
// This should error with https://nextjs.org/docs/messages/nested-styled-jsx-tags
// but instead it crashes SWC.
export default function Home() {
    return <div className={"jsx-915beb22be3340e9"}>
            <div className={"jsx-915beb22be3340e9"}>
                <div className={"jsx-915beb22be3340e9" + " " + "text animated"}>Text</div>

                <_JSXStyle id={"915beb22be3340e9"}>{".text.jsx-915beb22be3340e9:hover{color:#ff0!important}"}</_JSXStyle>
            </div>

            <style jsx>{`
        .text:hover {
          color: yellow !important;
        }
      `}</style>
        </div>;
}

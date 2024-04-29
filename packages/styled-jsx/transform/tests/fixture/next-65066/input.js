import React from "react";

// This should error with https://nextjs.org/docs/messages/nested-styled-jsx-tags
// but instead it crashes SWC.

export default function Home() {
    return (
        <div>
            <div>
                <div className={"text animated"}>Text</div>

                <style jsx>{`
          .text {
            color: blue;
          }
          .text:hover {
            color: green;
          }
        `}</style>
            </div>

            <style jsx>{`
        .text:hover {
          color: yellow !important;
        }
      `}</style>
        </div>
    );
}
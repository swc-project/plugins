import React from "react";

// This should error with https://nextjs.org/docs/messages/nested-styled-jsx-tags
// but does not.

export default function SimplePage() {
    return (
        <ComponentWithChildAsProp
            trigger={
                <div>
                    <div className={"text animated"}>Text</div>

                    <style jsx>{`
            .text {
              color: blue;

              // This should either get transpiled by SWC
              // or should cause a build error about
              // nested styled jsx tags.
              // https://nextjs.org/docs/messages/nested-styled-jsx-tags
              //
              // Instead, it causes a hydration error,
              // because & gets replaced with an &amp;
              // *Remove* to fix the hydration error.
              &:hover {
                color: red;
              }
            }
          `}</style>
                </div>
            }
        />
    );
}

const ComponentWithChildAsProp = ({
    trigger,
}) => {
    return <div>{trigger}</div>;
};
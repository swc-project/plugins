import _styled from "@emotion/styled/base";

function _EMOTION_STRINGIFIED_CSS_ERROR__() { return "You have tried to stringify object returned from `css` function. It isn't supposed to be used directly (e.g. as value of the `className` prop), but rather handed to emotion so it can handle it (e.g. as value of `css` prop)."; }

// @flow
import * as React from 'react';
import { jsx } from '@emotion/react';
const MyObject = {
  MyProperty: _styled("div", process.env.NODE_ENV === "production" ? {
    target: "eq7c75c0"
  } : {
    target: "eq7c75c0",
    label: "MyProperty"
  })(process.env.NODE_ENV === "production" ? {
    name: "3sn2xs",
    styles: "color:hotpink"
  } : {
    name: "3sn2xs",
    styles: "color:hotpink",
    map: "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uL19fdGVzdHNfXy9fX2ZpeHR1cmVzX18vb2JqZWN0LXByb3BlcnR5LmpzIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQU1jIiwiZmlsZSI6Ii4uLy4uLy4uL19fdGVzdHNfXy9fX2ZpeHR1cmVzX18vb2JqZWN0LXByb3BlcnR5LmpzIiwic291cmNlc0NvbnRlbnQiOlsiLy8gQGZsb3dcbmltcG9ydCAqIGFzIFJlYWN0IGZyb20gJ3JlYWN0J1xuaW1wb3J0IHsganN4IH0gZnJvbSAnQGVtb3Rpb24vcmVhY3QnXG5pbXBvcnQgc3R5bGVkIGZyb20gJ0BlbW90aW9uL3N0eWxlZCdcblxuY29uc3QgTXlPYmplY3QgPSB7XG4gIE15UHJvcGVydHk6IHN0eWxlZC5kaXYoeyBjb2xvcjogJ2hvdHBpbmsnIH0pXG59XG5cbmZ1bmN0aW9uIExvZ28ocHJvcHMpIHtcbiAgcmV0dXJuIDxNeU9iamVjdC5NeVByb3BlcnR5IC8+XG59XG4iXX0= */",
    toString: _EMOTION_STRINGIFIED_CSS_ERROR__
  })
};

function Logo(props) {
  return <MyObject.MyProperty />;
}

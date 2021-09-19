import _styled from "@emotion/styled/base";

function _EMOTION_STRINGIFIED_CSS_ERROR__() { return "You have tried to stringify object returned from `css` function. It isn't supposed to be used directly (e.g. as value of the `className` prop), but rather handed to emotion so it can handle it (e.g. as value of `css` prop)."; }

// @flow
const MyComponent = _styled("div", process.env.NODE_ENV === "production" ? {
  target: "em0vifp0"
} : {
  target: "em0vifp0",
  label: "MyComponent"
})(process.env.NODE_ENV === "production" ? {
  name: "3sn2xs",
  styles: "color:hotpink"
} : {
  name: "3sn2xs",
  styles: "color:hotpink",
  map: "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uL19fdGVzdHNfXy9fX2ZpeHR1cmVzX18vY29yZS13aXRoLWNvbXBvbmVudC5qcyJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiQUFHb0IiLCJmaWxlIjoiLi4vLi4vLi4vX190ZXN0c19fL19fZml4dHVyZXNfXy9jb3JlLXdpdGgtY29tcG9uZW50LmpzIiwic291cmNlc0NvbnRlbnQiOlsiLy8gQGZsb3dcbmltcG9ydCBzdHlsZWQgZnJvbSAnQGVtb3Rpb24vc3R5bGVkJ1xuXG5jb25zdCBNeUNvbXBvbmVudCA9IHN0eWxlZC5kaXYoeyBjb2xvcjogJ2hvdHBpbmsnIH0pXG5cbmNvbnN0IE90aGVyQ29tcG9uZW50ID0gTXlDb21wb25lbnQud2l0aENvbXBvbmVudCgnc2VjdGlvbicpXG4iXX0= */",
  toString: _EMOTION_STRINGIFIED_CSS_ERROR__
});

const OtherComponent = MyComponent.withComponent('section', process.env.NODE_ENV === "production" ? {
  target: "em0vifp1"
} : {
  target: "em0vifp1",
  label: "OtherComponent"
});

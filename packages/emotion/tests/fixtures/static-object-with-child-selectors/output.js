function _EMOTION_STRINGIFIED_CSS_ERROR__() { return "You have tried to stringify object returned from `css` function. It isn't supposed to be used directly (e.g. as value of the `className` prop), but rather handed to emotion so it can handle it (e.g. as value of `css` prop)."; }

/** @jsx jsx */
import { jsx } from '@emotion/react';

var _ref = process.env.NODE_ENV === "production" ? {
  name: "1t2q7lq",
  styles: "color:green;:hover{color:hotpink;}"
} : {
  name: "1vvht02-SomeComponent",
  styles: "color:green;:hover{color:hotpink;};label:SomeComponent;",
  map: "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uL19fdGVzdHNfXy9fX2ZpeHR1cmVzX18vc3RhdGljLW9iamVjdC13aXRoLWNoaWxkLXNlbGVjdG9ycy5qcyJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiQUFPTSIsImZpbGUiOiIuLi8uLi8uLi9fX3Rlc3RzX18vX19maXh0dXJlc19fL3N0YXRpYy1vYmplY3Qtd2l0aC1jaGlsZC1zZWxlY3RvcnMuanMiLCJzb3VyY2VzQ29udGVudCI6WyIvKiogQGpzeCBqc3ggKi9cblxuaW1wb3J0IHsganN4IH0gZnJvbSAnQGVtb3Rpb24vcmVhY3QnXG5cbmNvbnN0IFNvbWVDb21wb25lbnQgPSBwcm9wcyA9PiB7XG4gIHJldHVybiAoXG4gICAgPGRpdlxuICAgICAgY3NzPXt7XG4gICAgICAgIGNvbG9yOiAnZ3JlZW4nLFxuICAgICAgICAnOmhvdmVyJzoge1xuICAgICAgICAgIGNvbG9yOiAnaG90cGluaydcbiAgICAgICAgfVxuICAgICAgfX1cbiAgICAgIHsuLi5wcm9wc31cbiAgICAvPlxuICApXG59XG4iXX0= */",
  toString: _EMOTION_STRINGIFIED_CSS_ERROR__
};

const SomeComponent = props => {
  return <div css={_ref} {...props} />;
};

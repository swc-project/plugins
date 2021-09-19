function _EMOTION_STRINGIFIED_CSS_ERROR__() { return "You have tried to stringify object returned from `css` function. It isn't supposed to be used directly (e.g. as value of the `className` prop), but rather handed to emotion so it can handle it (e.g. as value of `css` prop)."; }

/** @jsx jsx */
import { jsx, css } from '@emotion/react';

var _ref = process.env.NODE_ENV === "production" ? {
  name: "3sn2xs",
  styles: "color:hotpink"
} : {
  name: "1wl9j1m-SomeComponent",
  styles: "color:hotpink;label:SomeComponent;",
  map: "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uLy4uL19fdGVzdHNfXy9jc3MtbWFjcm8vX19maXh0dXJlc19fL2FjdHVhbC1leHBlY3RlZC11c2FnZS5qcyJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiQUFLWSIsImZpbGUiOiIuLi8uLi8uLi8uLi9fX3Rlc3RzX18vY3NzLW1hY3JvL19fZml4dHVyZXNfXy9hY3R1YWwtZXhwZWN0ZWQtdXNhZ2UuanMiLCJzb3VyY2VzQ29udGVudCI6WyIvKiogQGpzeCBqc3ggKi9cbmltcG9ydCB7IGpzeCwgY3NzIH0gZnJvbSAnQGVtb3Rpb24vcmVhY3QnXG5cbmNvbnN0IFNvbWVDb21wb25lbnQgPSAoKSA9PiAoXG4gIDxkaXZcbiAgICBjc3M9e2Nzc2BcbiAgICAgIGNvbG9yOiBob3RwaW5rO1xuICAgIGB9XG4gIC8+XG4pXG4iXX0= */",
  toString: _EMOTION_STRINGIFIED_CSS_ERROR__
};

const SomeComponent = () => <div css={_ref} />;

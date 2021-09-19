function _EMOTION_STRINGIFIED_CSS_ERROR__() { return "You have tried to stringify object returned from `css` function. It isn't supposed to be used directly (e.g. as value of the `className` prop), but rather handed to emotion so it can handle it (e.g. as value of `css` prop)."; }

/** @jsx jsx */
import { Component } from 'react';
import { jsx, css } from '@emotion/react';

var _ref = process.env.NODE_ENV === "production" ? {
  name: "3sn2xs",
  styles: "color:hotpink"
} : {
  name: "1wl9j1m-SomeComponent",
  styles: "color:hotpink;label:SomeComponent;",
  map: "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uLy4uL19fdGVzdHNfXy9jc3MtbWFjcm8vX19maXh0dXJlc19fL2luc2lkZS1jbGFzcy5qcyJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiQUFRZ0IiLCJmaWxlIjoiLi4vLi4vLi4vLi4vX190ZXN0c19fL2Nzcy1tYWNyby9fX2ZpeHR1cmVzX18vaW5zaWRlLWNsYXNzLmpzIiwic291cmNlc0NvbnRlbnQiOlsiLyoqIEBqc3gganN4ICovXG5pbXBvcnQgeyBDb21wb25lbnQgfSBmcm9tICdyZWFjdCdcbmltcG9ydCB7IGpzeCwgY3NzIH0gZnJvbSAnQGVtb3Rpb24vcmVhY3QnXG5cbmNsYXNzIFNvbWVDb21wb25lbnQgZXh0ZW5kcyBDb21wb25lbnQge1xuICByZW5kZXIoKSB7XG4gICAgcmV0dXJuIChcbiAgICAgIDxkaXZcbiAgICAgICAgY3NzPXtjc3NgXG4gICAgICAgICAgY29sb3I6IGhvdHBpbms7XG4gICAgICAgIGB9XG4gICAgICAvPlxuICAgIClcbiAgfVxufVxuIl19 */",
  toString: _EMOTION_STRINGIFIED_CSS_ERROR__
};

class SomeComponent extends Component {
  render() {
    return <div css={_ref} />;
  }

}

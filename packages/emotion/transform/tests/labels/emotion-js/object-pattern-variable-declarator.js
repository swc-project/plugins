import { jsx as _jsx } from "react/jsx-runtime";
// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/object-pattern-variable-declarator.js
import { css } from "@emotion/react";
import { extractCritical } from "@emotion/server";
import React from "react";
import { renderToString } from "react-dom/server";
const { css: styles } = extractCritical(renderToString(/*#__PURE__*/ _jsx("div", {
    css: /*#__PURE__*/ css("color:hotpink;", "", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZW1vdGlvbi1qcy9vYmplY3QtcGF0dGVybi12YXJpYWJsZS1kZWNsYXJhdG9yLnRzeCIsInNvdXJjZXMiOlsiZW1vdGlvbi1qcy9vYmplY3QtcGF0dGVybi12YXJpYWJsZS1kZWNsYXJhdG9yLnRzeCJdLCJzb3VyY2VzQ29udGVudCI6WyIvLyBodHRwczovL2dpdGh1Yi5jb20vZW1vdGlvbi1qcy9lbW90aW9uL2Jsb2IvbWFpbi9wYWNrYWdlcy9iYWJlbC1wbHVnaW4vX190ZXN0c19fL2Nzcy1tYWNyby9fX2ZpeHR1cmVzX18vb2JqZWN0LXBhdHRlcm4tdmFyaWFibGUtZGVjbGFyYXRvci5qc1xuXG5pbXBvcnQgeyBjc3MgfSBmcm9tIFwiQGVtb3Rpb24vcmVhY3RcIjtcbmltcG9ydCB7IGV4dHJhY3RDcml0aWNhbCB9IGZyb20gXCJAZW1vdGlvbi9zZXJ2ZXJcIjtcbmltcG9ydCBSZWFjdCBmcm9tIFwicmVhY3RcIjtcbmltcG9ydCB7IHJlbmRlclRvU3RyaW5nIH0gZnJvbSBcInJlYWN0LWRvbS9zZXJ2ZXJcIjtcblxuY29uc3QgeyBjc3M6IHN0eWxlcyB9ID0gZXh0cmFjdENyaXRpY2FsKFxuICByZW5kZXJUb1N0cmluZyhcbiAgICA8ZGl2XG4gICAgICBjc3M9e2Nzc2BcbiAgICAgICAgY29sb3I6IGhvdHBpbms7XG4gICAgICBgfVxuICAgIC8+LFxuICApLFxuKTtcbiJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiQUFVVyJ9 */")
})));

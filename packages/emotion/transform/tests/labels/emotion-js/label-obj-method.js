// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/label-obj-method.js
import { jsx as _jsx } from "react/jsx-runtime";
import { css } from "@emotion/react";
const obj = {
    FooBar () {
        return /*#__PURE__*/ _jsx("div", {
            css: /*#__PURE__*/ css("background-color:hotpink;", "FooBar", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZW1vdGlvbi1qcy9sYWJlbC1vYmotbWV0aG9kLnRzeCIsInNvdXJjZXMiOlsiZW1vdGlvbi1qcy9sYWJlbC1vYmotbWV0aG9kLnRzeCJdLCJzb3VyY2VzQ29udGVudCI6WyIvLyBodHRwczovL2dpdGh1Yi5jb20vZW1vdGlvbi1qcy9lbW90aW9uL2Jsb2IvbWFpbi9wYWNrYWdlcy9iYWJlbC1wbHVnaW4vX190ZXN0c19fL2Nzcy1tYWNyby9fX2ZpeHR1cmVzX18vbGFiZWwtb2JqLW1ldGhvZC5qc1xuXG5pbXBvcnQgeyBjc3MgfSBmcm9tIFwiQGVtb3Rpb24vcmVhY3RcIjtcblxuY29uc3Qgb2JqID0ge1xuICBGb29CYXIoKSB7XG4gICAgcmV0dXJuIChcbiAgICAgIDxkaXZcbiAgICAgICAgY3NzPXtjc3NgXG4gICAgICAgICAgYmFja2dyb3VuZC1jb2xvcjogaG90cGluaztcbiAgICAgICAgYH1cbiAgICAgIC8+XG4gICAgKTtcbiAgfSxcbn07XG4iXSwibmFtZXMiOltdLCJyYW5nZU1hcHBpbmdzIjoiIiwibWFwcGluZ3MiOiJBQVFhIn0= */")
        });
    }
};

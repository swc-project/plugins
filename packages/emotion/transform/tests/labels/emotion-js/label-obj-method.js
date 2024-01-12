// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/label-obj-method.js
import { jsx as _jsx } from "react/jsx-runtime";
import { css } from "@emotion/react";
const obj = {
    FooBar () {
        return /*#__PURE__*/ _jsx("div", {
            css: /*#__PURE__*/ css("background-color:hotpink;", "FooBar", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZW1vdGlvbi1qcy9sYWJlbC1vYmotbWV0aG9kLnRzIiwic291cmNlcyI6WyJlbW90aW9uLWpzL2xhYmVsLW9iai1tZXRob2QudHMiXSwic291cmNlc0NvbnRlbnQiOlsiLy8gaHR0cHM6Ly9naXRodWIuY29tL2Vtb3Rpb24tanMvZW1vdGlvbi9ibG9iL21haW4vcGFja2FnZXMvYmFiZWwtcGx1Z2luL19fdGVzdHNfXy9jc3MtbWFjcm8vX19maXh0dXJlc19fL2xhYmVsLW9iai1tZXRob2QuanNcblxuaW1wb3J0IHsgY3NzIH0gZnJvbSBcIkBlbW90aW9uL3JlYWN0XCI7XG5cbmNvbnN0IG9iaiA9IHtcbiAgRm9vQmFyKCkge1xuICAgIHJldHVybiAoXG4gICAgICA8ZGl2XG4gICAgICAgIGNzcz17Y3NzYFxuICAgICAgICAgIGJhY2tncm91bmQtY29sb3I6IGhvdHBpbms7XG4gICAgICAgIGB9XG4gICAgICAvPlxuICAgICk7XG4gIH0sXG59O1xuIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQVFhIn0= */")
        });
    }
};

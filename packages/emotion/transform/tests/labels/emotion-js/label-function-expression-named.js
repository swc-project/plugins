// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/label-function-expression-named.js
import { css } from "@emotion/react";
const thing = function someName() {
  return /*#__PURE__*/ css(
    "color:hotpink;",
    "someName",
    "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZW1vdGlvbi1qcy9sYWJlbC1mdW5jdGlvbi1leHByZXNzaW9uLW5hbWVkLnRzIiwic291cmNlcyI6WyJlbW90aW9uLWpzL2xhYmVsLWZ1bmN0aW9uLWV4cHJlc3Npb24tbmFtZWQudHMiXSwic291cmNlc0NvbnRlbnQiOlsiLy8gaHR0cHM6Ly9naXRodWIuY29tL2Vtb3Rpb24tanMvZW1vdGlvbi9ibG9iL21haW4vcGFja2FnZXMvYmFiZWwtcGx1Z2luL19fdGVzdHNfXy9jc3MtbWFjcm8vX19maXh0dXJlc19fL2xhYmVsLWZ1bmN0aW9uLWV4cHJlc3Npb24tbmFtZWQuanNcblxuaW1wb3J0IHsgY3NzIH0gZnJvbSBcIkBlbW90aW9uL3JlYWN0XCI7XG5cbmNvbnN0IHRoaW5nID0gZnVuY3Rpb24gc29tZU5hbWUoKSB7XG4gIHJldHVybiBjc3NgXG4gICAgY29sb3I6IGhvdHBpbms7XG4gIGA7XG59O1xuIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQUtTIn0= */",
  );
};

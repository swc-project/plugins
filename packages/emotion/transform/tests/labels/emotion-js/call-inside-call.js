// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/call-inside-call.js
import { css } from "@emotion/react";
const thing = /*#__PURE__*/ css("display:flex;&:hover{", css`
      color: hotpink;
    `, ";}", "thing", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZW1vdGlvbi1qcy9jYWxsLWluc2lkZS1jYWxsLnRzIiwic291cmNlcyI6WyJlbW90aW9uLWpzL2NhbGwtaW5zaWRlLWNhbGwudHMiXSwic291cmNlc0NvbnRlbnQiOlsiLy8gaHR0cHM6Ly9naXRodWIuY29tL2Vtb3Rpb24tanMvZW1vdGlvbi9ibG9iL21haW4vcGFja2FnZXMvYmFiZWwtcGx1Z2luL19fdGVzdHNfXy9jc3MtbWFjcm8vX19maXh0dXJlc19fL2NhbGwtaW5zaWRlLWNhbGwuanNcblxuaW1wb3J0IHsgY3NzIH0gZnJvbSBcIkBlbW90aW9uL3JlYWN0XCI7XG5cbmNvbnN0IHRoaW5nID0gY3NzYFxuICBkaXNwbGF5OiBmbGV4O1xuICAmOmhvdmVyIHtcbiAgICAke2Nzc2BcbiAgICAgIGNvbG9yOiBob3RwaW5rO1xuICAgIGB9O1xuICB9XG5gO1xuIl0sIm5hbWVzIjpbXSwicmFuZ2VNYXBwaW5ncyI6IiIsIm1hcHBpbmdzIjoiQUFJYyJ9 */");

// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/call-expression.js
import { css } from "@emotion/react";
function doThing() {
    return /*#__PURE__*/ css({
        color: "hotpink"
    }, "label:doThing", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZW1vdGlvbi1qcy9jYWxsLWV4cHJlc3Npb24udHMiLCJzb3VyY2VzIjpbImVtb3Rpb24tanMvY2FsbC1leHByZXNzaW9uLnRzIl0sInNvdXJjZXNDb250ZW50IjpbIi8vIGh0dHBzOi8vZ2l0aHViLmNvbS9lbW90aW9uLWpzL2Vtb3Rpb24vYmxvYi9tYWluL3BhY2thZ2VzL2JhYmVsLXBsdWdpbi9fX3Rlc3RzX18vY3NzLW1hY3JvL19fZml4dHVyZXNfXy9jYWxsLWV4cHJlc3Npb24uanNcblxuaW1wb3J0IHsgY3NzIH0gZnJvbSBcIkBlbW90aW9uL3JlYWN0XCI7XG5cbmZ1bmN0aW9uIGRvVGhpbmcoKSB7XG4gIHJldHVybiBjc3MoeyBjb2xvcjogXCJob3RwaW5rXCIgfSk7XG59XG4iXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBS1MifQ== */");
}

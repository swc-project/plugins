// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/impure.js
import { css } from "@emotion/react";
function thing() {}
function doThing() {
    return /*#__PURE__*/ css("display:", thing(), ";", "doThing", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZW1vdGlvbi1qcy9pbXB1cmUudHMiLCJzb3VyY2VzIjpbImVtb3Rpb24tanMvaW1wdXJlLnRzIl0sInNvdXJjZXNDb250ZW50IjpbIi8vIGh0dHBzOi8vZ2l0aHViLmNvbS9lbW90aW9uLWpzL2Vtb3Rpb24vYmxvYi9tYWluL3BhY2thZ2VzL2JhYmVsLXBsdWdpbi9fX3Rlc3RzX18vY3NzLW1hY3JvL19fZml4dHVyZXNfXy9pbXB1cmUuanNcblxuaW1wb3J0IHsgY3NzIH0gZnJvbSBcIkBlbW90aW9uL3JlYWN0XCI7XG5cbmZ1bmN0aW9uIHRoaW5nKCkge31cblxuZnVuY3Rpb24gZG9UaGluZygpIHtcbiAgcmV0dXJuIGNzc2BcbiAgICBkaXNwbGF5OiAke3RoaW5nKCl9O1xuICBgO1xufVxuIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQU9TIn0= */");
}

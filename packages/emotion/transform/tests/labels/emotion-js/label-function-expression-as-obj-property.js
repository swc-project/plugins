// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/label-function-expression-as-obj-property.js
import { css } from "@emotion/react";
export const styles = {
  colorFn1: function () {
    return /*#__PURE__*/ css(
      "color:hotpink;",
      "colorFn1",
      "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZW1vdGlvbi1qcy9sYWJlbC1mdW5jdGlvbi1leHByZXNzaW9uLWFzLW9iai1wcm9wZXJ0eS50cyIsInNvdXJjZXMiOlsiZW1vdGlvbi1qcy9sYWJlbC1mdW5jdGlvbi1leHByZXNzaW9uLWFzLW9iai1wcm9wZXJ0eS50cyJdLCJzb3VyY2VzQ29udGVudCI6WyIvLyBodHRwczovL2dpdGh1Yi5jb20vZW1vdGlvbi1qcy9lbW90aW9uL2Jsb2IvbWFpbi9wYWNrYWdlcy9iYWJlbC1wbHVnaW4vX190ZXN0c19fL2Nzcy1tYWNyby9fX2ZpeHR1cmVzX18vbGFiZWwtZnVuY3Rpb24tZXhwcmVzc2lvbi1hcy1vYmotcHJvcGVydHkuanNcblxuaW1wb3J0IHsgY3NzIH0gZnJvbSBcIkBlbW90aW9uL3JlYWN0XCI7XG5cbmV4cG9ydCBjb25zdCBzdHlsZXMgPSB7XG4gIGNvbG9yRm4xOiBmdW5jdGlvbiAoKSB7XG4gICAgcmV0dXJuIGNzc2BcbiAgICAgIGNvbG9yOiBob3RwaW5rO1xuICAgIGA7XG4gIH0sXG59O1xuIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQU1XIn0= */",
    );
  },
};

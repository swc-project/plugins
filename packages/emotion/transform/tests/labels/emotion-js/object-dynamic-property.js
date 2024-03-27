// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/object-dynamic-property.js
import { css } from "@emotion/react";
function doThing() {
  return {
    [/*#__PURE__*/ css(
      {
        color: "hotpink",
      },
      "",
      "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZW1vdGlvbi1qcy9vYmplY3QtZHluYW1pYy1wcm9wZXJ0eS50cyIsInNvdXJjZXMiOlsiZW1vdGlvbi1qcy9vYmplY3QtZHluYW1pYy1wcm9wZXJ0eS50cyJdLCJzb3VyY2VzQ29udGVudCI6WyIvLyBodHRwczovL2dpdGh1Yi5jb20vZW1vdGlvbi1qcy9lbW90aW9uL2Jsb2IvbWFpbi9wYWNrYWdlcy9iYWJlbC1wbHVnaW4vX190ZXN0c19fL2Nzcy1tYWNyby9fX2ZpeHR1cmVzX18vb2JqZWN0LWR5bmFtaWMtcHJvcGVydHkuanNcblxuaW1wb3J0IHsgY3NzIH0gZnJvbSBcIkBlbW90aW9uL3JlYWN0XCI7XG5cbmZ1bmN0aW9uIGRvVGhpbmcoKSB7XG4gIHJldHVybiB7XG4gICAgW2Nzcyh7IGNvbG9yOiBcImhvdHBpbmtcIiB9KV06IFwiY29sZGJsdWVcIixcbiAgfTtcbn1cbiJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiQUFNSyJ9 */",
    )]: "coldblue",
  };
}

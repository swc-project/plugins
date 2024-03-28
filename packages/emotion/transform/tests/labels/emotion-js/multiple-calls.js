// https://github.com/emotion-js/emotion/blob/main/packages/babel-plugin/__tests__/css-macro/__fixtures__/multiple-calls.js
import { css } from "@emotion/react";
const thing = /*#__PURE__*/ css("color:hotpink;", "thing", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZW1vdGlvbi1qcy9tdWx0aXBsZS1jYWxscy50cyIsInNvdXJjZXMiOlsiZW1vdGlvbi1qcy9tdWx0aXBsZS1jYWxscy50cyJdLCJzb3VyY2VzQ29udGVudCI6WyIvLyBodHRwczovL2dpdGh1Yi5jb20vZW1vdGlvbi1qcy9lbW90aW9uL2Jsb2IvbWFpbi9wYWNrYWdlcy9iYWJlbC1wbHVnaW4vX190ZXN0c19fL2Nzcy1tYWNyby9fX2ZpeHR1cmVzX18vbXVsdGlwbGUtY2FsbHMuanNcblxuaW1wb3J0IHsgY3NzIH0gZnJvbSBcIkBlbW90aW9uL3JlYWN0XCI7XG5cbmNvbnN0IHRoaW5nID0gY3NzYFxuICBjb2xvcjogaG90cGluaztcbmA7XG5cbmNvbnN0IG90aGVyVGhpbmcgPSBjc3NgXG4gIGNvbG9yOiBncmVlbjtcbmA7XG4iXSwibmFtZXMiOltdLCJyYW5nZU1hcHBpbmdzIjoiIiwibWFwcGluZ3MiOiJBQUljIn0= */");
const otherThing = /*#__PURE__*/ css("color:green;", "otherThing", "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZW1vdGlvbi1qcy9tdWx0aXBsZS1jYWxscy50cyIsInNvdXJjZXMiOlsiZW1vdGlvbi1qcy9tdWx0aXBsZS1jYWxscy50cyJdLCJzb3VyY2VzQ29udGVudCI6WyIvLyBodHRwczovL2dpdGh1Yi5jb20vZW1vdGlvbi1qcy9lbW90aW9uL2Jsb2IvbWFpbi9wYWNrYWdlcy9iYWJlbC1wbHVnaW4vX190ZXN0c19fL2Nzcy1tYWNyby9fX2ZpeHR1cmVzX18vbXVsdGlwbGUtY2FsbHMuanNcblxuaW1wb3J0IHsgY3NzIH0gZnJvbSBcIkBlbW90aW9uL3JlYWN0XCI7XG5cbmNvbnN0IHRoaW5nID0gY3NzYFxuICBjb2xvcjogaG90cGluaztcbmA7XG5cbmNvbnN0IG90aGVyVGhpbmcgPSBjc3NgXG4gIGNvbG9yOiBncmVlbjtcbmA7XG4iXSwibmFtZXMiOltdLCJyYW5nZU1hcHBpbmdzIjoiIiwibWFwcGluZ3MiOiJBQVFtQiJ9 */");

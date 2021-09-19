import { css } from '@emotion/react';

function thing() {}

function doThing() {
  return /*#__PURE__*/css("display:", thing(), ";" + (process.env.NODE_ENV === "production" ? "" : ";label:doThing;"), process.env.NODE_ENV === "production" ? "" : "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uLy4uL19fdGVzdHNfXy9jc3MtbWFjcm8vX19maXh0dXJlc19fL2ltcHVyZS5qcyJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiQUFLWSIsImZpbGUiOiIuLi8uLi8uLi8uLi9fX3Rlc3RzX18vY3NzLW1hY3JvL19fZml4dHVyZXNfXy9pbXB1cmUuanMiLCJzb3VyY2VzQ29udGVudCI6WyJpbXBvcnQgeyBjc3MgfSBmcm9tICdAZW1vdGlvbi9yZWFjdCdcblxuZnVuY3Rpb24gdGhpbmcoKSB7IH1cblxuZnVuY3Rpb24gZG9UaGluZygpIHtcbiAgcmV0dXJuIGNzc2BcbiAgICBkaXNwbGF5OiAke3RoaW5nKCl9O1xuICBgXG59XG4iXX0= */");
}

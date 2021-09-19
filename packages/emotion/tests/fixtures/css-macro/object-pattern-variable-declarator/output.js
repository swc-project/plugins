function _EMOTION_STRINGIFIED_CSS_ERROR__() { return "You have tried to stringify object returned from `css` function. It isn't supposed to be used directly (e.g. as value of the `className` prop), but rather handed to emotion so it can handle it (e.g. as value of `css` prop)."; }

import { css } from '@emotion/react';
import { extractCritical } from '@emotion/server';
import React from 'react';
import { renderToString } from 'react-dom/server';
const {
  css: styles
} = extractCritical(renderToString(<div css={process.env.NODE_ENV === "production" ? {
  name: "3sn2xs",
  styles: "color:hotpink"
} : {
  name: "3sn2xs",
  styles: "color:hotpink",
  map: "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uLy4uL19fdGVzdHNfXy9jc3MtbWFjcm8vX19maXh0dXJlc19fL29iamVjdC1wYXR0ZXJuLXZhcmlhYmxlLWRlY2xhcmF0b3IuanMiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBUWMiLCJmaWxlIjoiLi4vLi4vLi4vLi4vX190ZXN0c19fL2Nzcy1tYWNyby9fX2ZpeHR1cmVzX18vb2JqZWN0LXBhdHRlcm4tdmFyaWFibGUtZGVjbGFyYXRvci5qcyIsInNvdXJjZXNDb250ZW50IjpbImltcG9ydCB7IGNzcyB9IGZyb20gJ0BlbW90aW9uL3JlYWN0J1xuaW1wb3J0IHsgZXh0cmFjdENyaXRpY2FsIH0gZnJvbSAnQGVtb3Rpb24vc2VydmVyJ1xuaW1wb3J0IFJlYWN0IGZyb20gJ3JlYWN0J1xuaW1wb3J0IHsgcmVuZGVyVG9TdHJpbmcgfSBmcm9tICdyZWFjdC1kb20vc2VydmVyJ1xuXG5jb25zdCB7IGNzczogc3R5bGVzIH0gPSBleHRyYWN0Q3JpdGljYWwoXG4gIHJlbmRlclRvU3RyaW5nKFxuICAgIDxkaXZcbiAgICAgIGNzcz17Y3NzYFxuICAgICAgICBjb2xvcjogaG90cGluaztcbiAgICAgIGB9XG4gICAgLz5cbiAgKVxuKVxuIl19 */",
  toString: _EMOTION_STRINGIFIED_CSS_ERROR__
}} />));

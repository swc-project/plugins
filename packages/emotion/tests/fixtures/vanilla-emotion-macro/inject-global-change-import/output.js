function _EMOTION_STRINGIFIED_CSS_ERROR__() { return "You have tried to stringify object returned from `css` function. It isn't supposed to be used directly (e.g. as value of the `className` prop), but rather handed to emotion so it can handle it (e.g. as value of `css` prop)."; }

import { injectGlobal as inject } from '@emotion/css';
inject(process.env.NODE_ENV === "production" ? {
  name: "59k0ad",
  styles: "body{margin:0;padding:0;&>div{display:flex;}}html{background:green;}"
} : {
  name: "59k0ad",
  styles: "body{margin:0;padding:0;&>div{display:flex;}}html{background:green;}",
  map: "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uLy4uL19fdGVzdHNfXy92YW5pbGxhLWVtb3Rpb24tbWFjcm8vX19maXh0dXJlc19fL2luamVjdC1nbG9iYWwtY2hhbmdlLWltcG9ydC5qcyJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiQUFFTSIsImZpbGUiOiIuLi8uLi8uLi8uLi9fX3Rlc3RzX18vdmFuaWxsYS1lbW90aW9uLW1hY3JvL19fZml4dHVyZXNfXy9pbmplY3QtZ2xvYmFsLWNoYW5nZS1pbXBvcnQuanMiLCJzb3VyY2VzQ29udGVudCI6WyJpbXBvcnQgeyBpbmplY3RHbG9iYWwgYXMgaW5qZWN0IH0gZnJvbSAnQGVtb3Rpb24vY3NzJ1xuXG5pbmplY3RgXG4gIGJvZHkge1xuICAgIG1hcmdpbjogMDtcbiAgICBwYWRkaW5nOiAwO1xuICAgICYgPiBkaXYge1xuICAgICAgZGlzcGxheTogZmxleDtcbiAgICB9XG4gIH1cbiAgaHRtbCB7XG4gICAgYmFja2dyb3VuZDogZ3JlZW47XG4gIH1cbmBcblxubGV0IGluamVjdEdsb2JhbCA9IHdpbmRvdy53aGF0ZXZlclxuXG5pbmplY3RHbG9iYWxgXG4gIGJvZHkge1xuICAgIG1hcmdpbjogMDtcbiAgICBwYWRkaW5nOiAwO1xuICAgICYgPiBkaXYge1xuICAgICAgZGlzcGxheTogZmxleDtcbiAgICB9XG4gIH1cbiAgaHRtbCB7XG4gICAgYmFja2dyb3VuZDogZ3JlZW47XG4gIH1cbmBcbiJdfQ== */",
  toString: _EMOTION_STRINGIFIED_CSS_ERROR__
});
let injectGlobal = window.whatever;
injectGlobal`
  body {
    margin: 0;
    padding: 0;
    & > div {
      display: flex;
    }
  }
  html {
    background: green;
  }
`;

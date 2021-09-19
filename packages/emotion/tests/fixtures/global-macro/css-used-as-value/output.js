function _EMOTION_STRINGIFIED_CSS_ERROR__() { return "You have tried to stringify object returned from `css` function. It isn't supposed to be used directly (e.g. as value of the `className` prop), but rather handed to emotion so it can handle it (e.g. as value of `css` prop)."; }

import * as React from 'react';
import { Global, css } from '@emotion/react'; // this gets ignored by Global macro, but it tests that this combination doesn't crash or something

var _ref = process.env.NODE_ENV === "production" ? {
  name: "3sn2xs",
  styles: "color:hotpink"
} : {
  name: "3sn2xs",
  styles: "color:hotpink",
  map: "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uLy4uL19fdGVzdHNfXy9nbG9iYWwtbWFjcm8vX19maXh0dXJlc19fL2Nzcy11c2VkLWFzLXZhbHVlLmpzIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQUlxQyIsImZpbGUiOiIuLi8uLi8uLi8uLi9fX3Rlc3RzX18vZ2xvYmFsLW1hY3JvL19fZml4dHVyZXNfXy9jc3MtdXNlZC1hcy12YWx1ZS5qcyIsInNvdXJjZXNDb250ZW50IjpbImltcG9ydCAqIGFzIFJlYWN0IGZyb20gJ3JlYWN0J1xuaW1wb3J0IHsgR2xvYmFsLCBjc3MgfSBmcm9tICdAZW1vdGlvbi9yZWFjdCdcblxuLy8gdGhpcyBnZXRzIGlnbm9yZWQgYnkgR2xvYmFsIG1hY3JvLCBidXQgaXQgdGVzdHMgdGhhdCB0aGlzIGNvbWJpbmF0aW9uIGRvZXNuJ3QgY3Jhc2ggb3Igc29tZXRoaW5nXG5leHBvcnQgZGVmYXVsdCAoKSA9PiA8R2xvYmFsIHN0eWxlcz17Y3NzKHsgY29sb3I6ICdob3RwaW5rJyB9KX0gLz5cbiJdfQ== */",
  toString: _EMOTION_STRINGIFIED_CSS_ERROR__
};

export default (() => <Global styles={_ref} />);

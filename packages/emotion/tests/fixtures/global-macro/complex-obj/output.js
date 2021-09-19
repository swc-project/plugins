import { css as _css } from "@emotion/react";
import * as React from 'react';
import { Global } from '@emotion/react';

const getBgColor = () => ({
  backgroundColor: '#fff'
});

export default (() => <Global styles={/*#__PURE__*/_css({
  color: 'hotpink',
  ...getBgColor()
}, process.env.NODE_ENV === "production" ? "" : "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uLy4uL19fdGVzdHNfXy9nbG9iYWwtbWFjcm8vX19maXh0dXJlc19fL2NvbXBsZXgtb2JqLmpzIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQUs2QiIsImZpbGUiOiIuLi8uLi8uLi8uLi9fX3Rlc3RzX18vZ2xvYmFsLW1hY3JvL19fZml4dHVyZXNfXy9jb21wbGV4LW9iai5qcyIsInNvdXJjZXNDb250ZW50IjpbImltcG9ydCAqIGFzIFJlYWN0IGZyb20gJ3JlYWN0J1xuaW1wb3J0IHsgR2xvYmFsIH0gZnJvbSAnQGVtb3Rpb24vcmVhY3QnXG5cbmNvbnN0IGdldEJnQ29sb3IgPSAoKSA9PiAoeyBiYWNrZ3JvdW5kQ29sb3I6ICcjZmZmJyB9KVxuXG5leHBvcnQgZGVmYXVsdCAoKSA9PiA8R2xvYmFsIHN0eWxlcz17eyBjb2xvcjogJ2hvdHBpbmsnLCAuLi5nZXRCZ0NvbG9yKCkgfX0gLz5cbiJdfQ== */")} />);

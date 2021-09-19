import { css as _css } from "@emotion/react";

/** @jsx someJsx */
import { someJsx } from 'package-two';

const SomeComponent = props => <div css={/*#__PURE__*/_css({
  color: window
}, process.env.NODE_ENV === "production" ? "" : ";label:SomeComponent;", process.env.NODE_ENV === "production" ? "" : "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uLy4uL19fdGVzdHNfXy9pbXBvcnQtbWFwcGluZy9fX2ZpeHR1cmVzX18vanN4LmpzIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQUdvQyIsImZpbGUiOiIuLi8uLi8uLi8uLi9fX3Rlc3RzX18vaW1wb3J0LW1hcHBpbmcvX19maXh0dXJlc19fL2pzeC5qcyIsInNvdXJjZXNDb250ZW50IjpbIi8qKiBAanN4IHNvbWVKc3ggKi9cbmltcG9ydCB7IHNvbWVKc3ggfSBmcm9tICdwYWNrYWdlLXR3bydcblxuY29uc3QgU29tZUNvbXBvbmVudCA9IHByb3BzID0+IDxkaXYgY3NzPXt7IGNvbG9yOiB3aW5kb3cgfX0gey4uLnByb3BzfSAvPlxuIl19 */")} {...props} />;

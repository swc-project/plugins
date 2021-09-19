import { css as _css } from "@emotion/react";

/** @jsx jsx */
import { jsx } from '@emotion/react';

const SomeComponent = props => <div css={/*#__PURE__*/_css({
  backgroundColor: window.something
}, process.env.NODE_ENV === "production" ? "" : ";label:SomeComponent;", process.env.NODE_ENV === "production" ? "" : "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uLy4uL19fdGVzdHNfXy9zb3VyY2UtbWFwcy9fX2ZpeHR1cmVzX18vY3NzLXByb3AtZHluYW1pYy5qcyJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiQUFLSSIsImZpbGUiOiIuLi8uLi8uLi8uLi9fX3Rlc3RzX18vc291cmNlLW1hcHMvX19maXh0dXJlc19fL2Nzcy1wcm9wLWR5bmFtaWMuanMiLCJzb3VyY2VzQ29udGVudCI6WyIvKiogQGpzeCBqc3ggKi9cbmltcG9ydCB7IGpzeCB9IGZyb20gJ0BlbW90aW9uL3JlYWN0J1xuXG5jb25zdCBTb21lQ29tcG9uZW50ID0gcHJvcHMgPT4gKFxuICA8ZGl2XG4gICAgY3NzPXt7XG4gICAgICBiYWNrZ3JvdW5kQ29sb3I6IHdpbmRvdy5zb21ldGhpbmdcbiAgICB9fVxuICAgIHsuLi5wcm9wc31cbiAgLz5cbilcbiJdfQ== */")} {...props} />;

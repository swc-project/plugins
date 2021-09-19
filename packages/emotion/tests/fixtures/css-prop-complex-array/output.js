/** @jsx jsx */
import { jsx, Global } from '@emotion/react';

const css1 = theme => ({
  backgroundColor: theme.bgColor
});

const css2 = theme => ({
  padding: theme.spacing.small
});

function SomeComponent(props) {
  return <div css={[css1, css2, process.env.NODE_ENV === "production" ? "" : ";label:SomeComponent;", process.env.NODE_ENV === "production" ? "" : "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uL19fdGVzdHNfXy9fX2ZpeHR1cmVzX18vY3NzLXByb3AtY29tcGxleC1hcnJheS5qcyJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiQUFPYyIsImZpbGUiOiIuLi8uLi8uLi9fX3Rlc3RzX18vX19maXh0dXJlc19fL2Nzcy1wcm9wLWNvbXBsZXgtYXJyYXkuanMiLCJzb3VyY2VzQ29udGVudCI6WyIvKiogQGpzeCBqc3ggKi9cbmltcG9ydCB7IGpzeCwgR2xvYmFsIH0gZnJvbSAnQGVtb3Rpb24vcmVhY3QnXG5cbmNvbnN0IGNzczEgPSB0aGVtZSA9PiAoeyBiYWNrZ3JvdW5kQ29sb3I6IHRoZW1lLmJnQ29sb3IgfSlcbmNvbnN0IGNzczIgPSB0aGVtZSA9PiAoeyBwYWRkaW5nOiB0aGVtZS5zcGFjaW5nLnNtYWxsIH0pXG5cbmZ1bmN0aW9uIFNvbWVDb21wb25lbnQocHJvcHMpIHtcbiAgcmV0dXJuIDxkaXYgY3NzPXtbY3NzMSwgY3NzMl19PnsnRW1vdGlvbid9PC9kaXY+XG59XG4iXX0= */"]}>{'Emotion'}</div>;
}

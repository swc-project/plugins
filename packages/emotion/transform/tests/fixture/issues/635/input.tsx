import { css } from "@emotion/react";
import { forwardRef } from "react";

const styles = {
  row: (theme) => css({ display: "grid", gap: theme.spacing.sm }),
};

export const RowWithSpread = forwardRef((props, ref) => (
  <div ref={ref} css={[styles.row, {}]} {...props} />
));

export function RowWithoutSpread() {
  return <div css={[styles.row, {}]} />;
}

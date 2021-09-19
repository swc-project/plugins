function _EMOTION_STRINGIFIED_CSS_ERROR__() { return "You have tried to stringify object returned from `css` function. It isn't supposed to be used directly (e.g. as value of the `className` prop), but rather handed to emotion so it can handle it (e.g. as value of `css` prop)."; }

/** @jsx jsx */
import { jsx, css } from '@emotion/react';

var _ref = process.env.NODE_ENV === "production" ? {
  name: "3sn2xs",
  styles: "color:hotpink"
} : {
  name: "1wl9j1m-SomeComponent",
  styles: "color:hotpink;label:SomeComponent;",
  map: "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uL19fdGVzdHNfXy9fX2ZpeHR1cmVzX18vY29yZS13aXRoLWNzcy1pbXBvcnQuanMiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBS1kiLCJmaWxlIjoiLi4vLi4vLi4vX190ZXN0c19fL19fZml4dHVyZXNfXy9jb3JlLXdpdGgtY3NzLWltcG9ydC5qcyIsInNvdXJjZXNDb250ZW50IjpbIi8qKiBAanN4IGpzeCAqL1xuaW1wb3J0IHsganN4LCBjc3MgfSBmcm9tICdAZW1vdGlvbi9yZWFjdCdcblxuY29uc3QgU29tZUNvbXBvbmVudCA9IHByb3BzID0+IChcbiAgPGRpdlxuICAgIGNzcz17Y3NzYFxuICAgICAgY29sb3I6IGhvdHBpbms7XG4gICAgYH1cbiAgICB7Li4ucHJvcHN9XG4gIC8+XG4pXG4iXX0= */",
  toString: _EMOTION_STRINGIFIED_CSS_ERROR__
};

const SomeComponent = props => <div css={_ref} {...props} />;

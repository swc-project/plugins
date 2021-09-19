function _EMOTION_STRINGIFIED_CSS_ERROR__() { return "You have tried to stringify object returned from `css` function. It isn't supposed to be used directly (e.g. as value of the `className` prop), but rather handed to emotion so it can handle it (e.g. as value of `css` prop)."; }

import styled from '@emotion/native';

/*#__PURE__*/
styled.View("color:hotpink;");

/*#__PURE__*/
styled.View("");

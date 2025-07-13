import React from "react";

export const A = () => {
  return <div>real a</div>;
};

const B = () => {
  return <A />;
};

export default B;

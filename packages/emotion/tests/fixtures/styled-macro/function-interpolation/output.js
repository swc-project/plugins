import _styled from "@emotion/styled/base";

let Avatar = _styled("img", process.env.NODE_ENV === "production" ? {
  target: "e15koq740"
} : {
  target: "e15koq740",
  label: "Avatar"
})("width:96px;height:96px;border-radius:", props => props.theme.borderRadius, ";border:1px solid ", props => props.theme.borderColor, ";" + (process.env.NODE_ENV === "production" ? "" : "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uLy4uL19fdGVzdHNfXy9zdHlsZWQtbWFjcm8vX19maXh0dXJlc19fL2Z1bmN0aW9uLWludGVycG9sYXRpb24uanMiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBRXVCIiwiZmlsZSI6Ii4uLy4uLy4uLy4uL19fdGVzdHNfXy9zdHlsZWQtbWFjcm8vX19maXh0dXJlc19fL2Z1bmN0aW9uLWludGVycG9sYXRpb24uanMiLCJzb3VyY2VzQ29udGVudCI6WyJpbXBvcnQgc3R5bGVkIGZyb20gJ0BlbW90aW9uL3N0eWxlZCdcblxubGV0IEF2YXRhciA9IHN0eWxlZC5pbWdgXG4gIHdpZHRoOiA5NnB4O1xuICBoZWlnaHQ6IDk2cHg7XG4gIGJvcmRlci1yYWRpdXM6ICR7cHJvcHMgPT4gcHJvcHMudGhlbWUuYm9yZGVyUmFkaXVzfTtcbiAgYm9yZGVyOiAxcHggc29saWQgJHtwcm9wcyA9PiBwcm9wcy50aGVtZS5ib3JkZXJDb2xvcn07XG5gXG4iXX0= */"));

import * as React from 'react';
import { Global } from '@emotion/react';

const getBgColor = () => ({
  backgroundColor: '#fff'
});

export default (() => <Global styles={["color:hotpink;", getBgColor(), process.env.NODE_ENV === "production" ? "" : "/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi4uLy4uLy4uLy4uL19fdGVzdHNfXy9nbG9iYWwtbWFjcm8vX19maXh0dXJlc19fL2NvbXBsZXgtYXJyYXkuanMiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBSzZCIiwiZmlsZSI6Ii4uLy4uLy4uLy4uL19fdGVzdHNfXy9nbG9iYWwtbWFjcm8vX19maXh0dXJlc19fL2NvbXBsZXgtYXJyYXkuanMiLCJzb3VyY2VzQ29udGVudCI6WyJpbXBvcnQgKiBhcyBSZWFjdCBmcm9tICdyZWFjdCdcbmltcG9ydCB7IEdsb2JhbCB9IGZyb20gJ0BlbW90aW9uL3JlYWN0J1xuXG5jb25zdCBnZXRCZ0NvbG9yID0gKCkgPT4gKHsgYmFja2dyb3VuZENvbG9yOiAnI2ZmZicgfSlcblxuZXhwb3J0IGRlZmF1bHQgKCkgPT4gPEdsb2JhbCBzdHlsZXM9e1t7IGNvbG9yOiAnaG90cGluaycgfSwgZ2V0QmdDb2xvcigpXX0gLz5cbiJdfQ== */"]} />);

import styled from "@emotion/styled";

const opts = { shouldForwardProp: (p: string) => !p.startsWith("$") };
const config = { opts };

const IdentTpl = styled("div", opts)`color: red;`;
const IdentCall = styled("div", opts)({ color: "red" });
const MemberTpl = styled("div", config.opts)`color: red;`;
const MemberCall = styled("div", config.opts)({ color: "red" });

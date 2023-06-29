import { Translate } from "fusion-plugin-i18n-react";
export default function() {
    return <>
      <span id="dont-include"/>
      <Translate random={"test"} id="test"/>
      <Translate id="test2"/>
    </>;
}

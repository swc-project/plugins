import _fusionPluginI18nChunkTranslationMap from "virtual:fusion-vite-i18n-map";
import { Translate } from "fusion-plugin-i18n-react";
export default function() {
    return <>

      <span id="dont-include"/>

      <Translate random={"test"} id="test"/>

      <Translate id="test2"/>

    </>;
}
_fusionPluginI18nChunkTranslationMap.add("/path/to/file.js", [
    "vite-i18n-chunk"
], [
    "test",
    "test2"
]);

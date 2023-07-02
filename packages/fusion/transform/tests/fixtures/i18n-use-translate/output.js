import _fusionPluginI18nChunkTranslationMap from "virtual:fusion-vite-i18n-map";
import { useTranslations } from "fusion-plugin-i18n-react";
export default function () {
  const translate = useTranslations();
  translate("static");
}
_fusionPluginI18nChunkTranslationMap.add("/path/to/file.js", [
    "vite-i18n-chunk"
], [
    "static",
]);

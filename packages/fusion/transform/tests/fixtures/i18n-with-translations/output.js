import _fusionPluginI18nChunkTranslationMap from "virtual:fusion-vite-i18n-map";
import { withTranslations } from 'fusion-plugin-i18n-react';
export default withTranslations(['test', 'foo'])(({
  translate
}) => {
  return <input placeholder={translate('test', {
    name: 'world'
  })} />;
});
_fusionPluginI18nChunkTranslationMap.add("/path/to/file.js", [
  "vite-i18n-chunk"
], [
  "foo",
  "test"
]);

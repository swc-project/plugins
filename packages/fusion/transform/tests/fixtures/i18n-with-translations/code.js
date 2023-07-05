import { withTranslations } from 'fusion-plugin-i18n-react';

export default withTranslations(['test', 'foo'])(({translate}) => {
  return <input placeholder={translate('test', {name: 'world'})} />;
});

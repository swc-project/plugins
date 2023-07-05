import { useTranslations } from "fusion-plugin-i18n-react";

export default function () {
  const translate = useTranslations();
  translate("static");
  translate(`prefix.${"foo"}.mid.${"baz"}`);
}

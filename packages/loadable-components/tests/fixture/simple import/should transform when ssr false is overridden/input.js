import loadable from "@loadable/component";

const opts = { ssr: true };

loadable(() => import("./OtherComponent"), { ssr: false, ...opts });

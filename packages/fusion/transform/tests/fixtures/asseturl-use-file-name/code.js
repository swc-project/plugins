import { assetUrl } from "fusion-core";

const Test = assetUrl("./foo.jpg");
const Test2 = assetUrl("./foo-bar.jpg");
const Test3 = assetUrl("./foo@bar.jpg");
const Test4 = assetUrl("./foo.jpg");

import { markAsPure } from "@swc/experimental-inject-imports";

markAsPure(console.log("test!"));

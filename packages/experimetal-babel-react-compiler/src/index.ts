import { transformSync as innerTransform } from "@babel/core";
import reactCompilerPlugin from "babel-plugin-react-compiler";

const reactCompilerConfig = {
  target: "19",
};

export function transform({ code, map }: { code: string; map?: string }) {
  return innerTransform(code, {
    plugins: [[reactCompilerPlugin, reactCompilerConfig]],
  });
}

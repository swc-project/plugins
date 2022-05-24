export interface RelayPluginOptions {
  rootDir: string;
  language: "typescript" | "flow";
  pagesDir?: string;
  artifactDirectory?: string;
}

export default function relay(
  options: RelayPluginOptions
): [string, Record<string, any>] {
  return ["@swc/plugin-relay", options];
}

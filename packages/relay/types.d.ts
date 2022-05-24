declare module "@swc/plugin-relay" {
  export interface Config {
    rootDir: string;
    language: "typescript" | "flow";
  }
}

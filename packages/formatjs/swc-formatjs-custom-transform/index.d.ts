/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface TransformOutput {
  code: string;
  map?: string;
}
export function transformSync(
  s: string,
  isModule: boolean,
  opts: Buffer,
  instrumentOpts: Buffer,
): TransformOutput;
export type JsCompiler = Compiler;
export class Compiler {
  constructor();
}

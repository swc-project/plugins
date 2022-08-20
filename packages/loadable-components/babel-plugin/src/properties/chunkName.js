import vm from 'vm'
import { getImportArg } from '../util'

const JS_PATH_REGEXP = /^[./]+|(\.js$)/g
const MATCH_LEFT_HYPHENS_REPLACE_REGEX = /^-/g
// https://github.com/webpack/webpack/blob/master/lib/Template.js
const WEBPACK_CHUNK_NAME_REGEXP = /webpackChunkName/
const WEBPACK_PATH_NAME_NORMALIZE_REPLACE_REGEX = /[^a-zA-Z0-9_!§$()=\-^°]+/g
const WEBPACK_MATCH_PADDED_HYPHENS_REPLACE_REGEX = /^-|-$/g


function moduleToChunk(str) {
  if (typeof str !== 'string') return ''
  return str
    .replace(JS_PATH_REGEXP, '')
    .replace(WEBPACK_PATH_NAME_NORMALIZE_REPLACE_REGEX, '-')
    .replace(WEBPACK_MATCH_PADDED_HYPHENS_REPLACE_REGEX, '')
}

function replaceQuasi(str, stripLeftHyphen) {
  if (!str) return ''
  const result = str.replace(WEBPACK_PATH_NAME_NORMALIZE_REPLACE_REGEX, '-')
  if (!stripLeftHyphen) return result
  return result.replace(MATCH_LEFT_HYPHENS_REPLACE_REGEX, '')
}

export default function chunkNameProperty({ types: t }) {


}

//! Port of https://github.com/styled-components/babel-plugin-styled-components/blob/a20c3033508677695953e7a434de4746168eeb4e/src/visitors/transpileCssProp.js

use swc_ecmascript::visit::{as_folder, noop_visit_mut_type, Fold, VisitMut};

pub fn transpile_css_prop() -> impl Fold + VisitMut {
    as_folder(TranspileCssProp)
}

struct TranspileCssProp;

impl VisitMut for TranspileCssProp {
    noop_visit_mut_type!();
}

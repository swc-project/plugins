use swc_ecmascript::visit::{Fold, VisitMut};
use visitors::template_literals::transpile::transpile_css_prop;

mod css;
mod utils;
mod visitors;

pub fn styled_components() -> impl Fold + VisitMut {
    transpile_css_prop()
}

use swc_ecmascript::visit::{as_folder, noop_visit_mut_type, Fold, VisitMut};

pub(crate) fn display_name_and_id() -> impl Fold + VisitMut {
    as_folder(DisplayNameAndId::default())
}

#[derive(Debug, Default)]
struct DisplayNameAndId {}

impl VisitMut for DisplayNameAndId {
    noop_visit_mut_type!();
}

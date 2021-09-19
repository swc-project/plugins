use swc_ecma_visit::{as_folder, noop_visit_mut_type, Fold, VisitMut};

pub fn emotion_plugin() -> impl VisitMut + Fold {
    as_folder(Emotion {})
}

struct Emotion {}

impl VisitMut for Emotion {
    // Reduce binary size.
    noop_visit_mut_type!();
}

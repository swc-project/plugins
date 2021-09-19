use swc_ecma_visit::{as_folder, noop_visit_mut_type, Fold, VisitMut};

pub fn emotion_plugin(config: Config) -> impl VisitMut + Fold {
    as_folder(Emotion {
        config,
        state: Default::default(),
    })
}

#[derive(Debug)]
pub struct Config {}

#[derive(Debug, Default)]
struct State {}

struct Emotion {
    #[allow(dead_code)]
    config: Config,
    state: State,
}

impl VisitMut for Emotion {
    // Reduce binary size.
    noop_visit_mut_type!();
}

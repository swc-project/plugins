use super::State;
use swc_common::DUMMY_SP;
use swc_ecmascript::{
    ast::{Invalid, Program},
    visit::{noop_visit_type, Visit, VisitWith},
};

pub fn analyze(program: &Program) -> State {
    let mut v = Analyzer {
        state: State {
            styled_required: Default::default(),
            imported_local_name: Default::default(),
            import_name_cache: Default::default(),
        },
    };

    program.visit_with(&Invalid { span: DUMMY_SP }, &mut v);

    v.state
}

struct Analyzer {
    state: State,
}

impl Visit for Analyzer {
    noop_visit_type!();
}

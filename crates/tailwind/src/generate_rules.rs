use indexmap::IndexSet;

use crate::{base::Candidate, context::Context};

pub(crate) fn generate_rules(
    candidates: &IndexSet<Candidate, ahash::RandomState>,
    context: &mut Context,
) {
    for candidate in candidates {}
}

fn resolve_matches(candidate: &Candidate, context: &mut Context) -> Vec<MatchResult> {
    vec![]
}

struct MatchResult {}

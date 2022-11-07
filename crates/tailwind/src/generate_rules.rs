use indexmap::IndexSet;

use crate::{base::Candidate, context::Context};

pub(crate) fn generate_rules(
    candidates: &IndexSet<Candidate, ahash::RandomState>,
    context: &mut Context,
) {
    for candidate in candidates {}
}

fn resolve_matches(
    candidate: &Candidate,
    context: &mut Context,
    original: Option<&Candidate>,
) -> Vec<MatchResult> {
    let original = original.unwrap_or(candidate);

    let separator = context.tailwind_config.separator;

    vec![]
}

struct MatchResult {}

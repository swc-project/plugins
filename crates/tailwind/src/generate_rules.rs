use indexmap::IndexSet;
use swc_atoms::Atom;

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

    let separator = context.tailwind_config.separator.clone();

    let (mut class_candidate, variants) = {
        let mut v = split_with_separator(candidate, separator);
        let c = v.pop().unwrap();
        v.reverse();
        (c, v)
    };
    let mut important = false;

    if class_candidate.starts_with('!') {
        important = true;
        class_candidate = class_candidate[1..].to_string();
    }

    vec![]
}

fn split_with_separator(candidate: &Candidate, separator: Atom) -> Vec<String> {
    todo!()
}

struct MatchResult {}

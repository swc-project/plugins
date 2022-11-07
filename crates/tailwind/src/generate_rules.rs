use indexmap::IndexSet;
use swc_atoms::Atom;
use swc_core::common::collections::{AHashMap, AHashSet};

use crate::{
    base::{Candidate, Modifier, Plugin, Sort},
    context::Context,
};

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
    let mut result = vec![];

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

    // TODO(kdy1): Port

    if is_variant_grouping_enabled
        && class_candidate.starts_with('(')
        && class_candidate.ends_with(')')
    {
        let base = {
            let mut v = variants.clone();
            v.reverse();
            v.join(&separator)
        };

        for part in split_at_top_level_only(&class_candidate[1..class_candidate.len() - 1], ",") {
            result.extend(resolve_matches(
                base + &*separator + part,
                context,
                original,
            ));
        }
    }

    for matched_plugins in resolve_matched_plugins(&class_candidate, context) {
        let mut matches = vec![];
        let mut type_by_matches = AHashMap::default();

        let (plugins, modifier) = matched_plugins;
        let is_only_plugin = plugins.len() == 1;

        for (sort, plugin) in plugins {
            let mut matches_per_plugin = vec![];
        }
    }

    result
}

fn resolve_matched_plugins(class_candidate: &str, context: &mut Context) -> Vec<MatchedPlugin> {
    todo!()
}

type MatchedPlugin = (Vec<(Sort, Plugin)>, Modifier);

fn split_with_separator(candidate: &Candidate, separator: Atom) -> Vec<String> {
    todo!()
}

struct MatchResult {}

use indexmap::IndexSet;

use crate::{base::Candidate, context::Context};

pub(crate) fn generate_rules(
    candidates: &IndexSet<Candidate, ahash::RandomState>,
    context: &mut Context,
) {
    for candidate in candidates {}
}

// fn resolve_matches(
//     candidate: &Candidate,
//     context: &mut Context,
//     original: Option<&Candidate>,
// ) -> Vec<MatchResult> {
//     let mut result = vec![];

//     let original = original.unwrap_or(candidate);

//     let separator = context.tailwind_config.separator.clone();

//     let (mut class_candidate, variants) = {
//         let mut v = split_with_separator(candidate, &separator);
//         let c = v.pop().unwrap();
//         v.reverse();
//         (c, v)
//     };
//     let mut important = false;

//     if class_candidate.starts_with('!') {
//         important = true;
//         class_candidate = class_candidate[1..];
//     }

//     // TODO(kdy1): Port

//     if is_variant_grouping_enabled
//         && class_candidate.starts_with('(')
//         && class_candidate.ends_with(')')
//     {
//         let base = {
//             let mut v = variants.clone();
//             v.reverse();
//             v.join(&separator)
//         };

//         for part in
// split_at_top_level_only(&class_candidate[1..class_candidate.len() - 1], ",")
// {             result.extend(resolve_matches(
//                 base + &*separator + part,
//                 context,
//                 Some(original),
//             ));
//         }
//     }

//     for matched_plugins in resolve_matched_plugins(&class_candidate, context)
// {         let mut matches = vec![];
//         let mut type_by_matches = AHashMap::default();

//         let (plugins, modifier) = matched_plugins;
//         let is_only_plugin = plugins.len() == 1;

//         for (sort, plugin) in plugins {
//             let mut matches_per_plugin = vec![];
//             match plugin {
//                 Plugin::Function(plugin) => {
//                     for rule_set in plugin(&modifier, &PluginContext {
// is_only_plugin }) {                         let (rules, options) =
// parse_rules(rule_set, context.postcss_node_cache);
// for rule in rules {                             let mut obj = sort.clone();
//                             sort.options =
// obj.options.into_iter().chain(options).collect();
// matches_per_plugin.push((obj, rule));                         }
//                     }
//                 }
//                 Plugin::Str(plugin) if modifier == "DEFAULT" || modifier ==
// "-DEFAULT" => {                     let rule_set = plugin;

//                     let (rules, options) = parse_rules(rule_set,
// context.postcss_node_cache);                     for rule in rules {
//                         let mut obj = sort.clone();
//                         sort.options =
// obj.options.into_iter().chain(options).collect();
// matches_per_plugin.push((obj, rule));                     }
//                 }
//                 _ => {}
//             }

//             if matches_per_plugin.len() > 0 {
//                 let matching_types = get_matching_types(
//                     sort.options.get("types").cloned().unwrap_or_default(),
//                     modifier,
//                     sort.options,
//                     context.tailwind_config,
//                 )
//                 .into_iter()
//                 .map(|(_, v)| v)
//                 .collect::<Vec<_>>();

//                 if matching_types.len() > 0 {
//                     type_by_matches.insert(sort, matching_types);
//                 }

//                 matches.push(matches_per_plugin);
//             }
//         }

//         if is_arbitrary_value(&modifier) {
//             if matches.len() > 1 {
//                 // Partition plugins in 2 categories so that we can start
//                 // searching in the plugins that don't have
//                 // `any` as a type first.
//             }

//             matches = matches.map(|list| list.filter(|m|
// is_parsable_node(m[1])));         }
//     }

//     result
// }

// fn resolve_matched_plugins(class_candidate: &str, context: &mut Context) ->
// Vec<MatchedPlugin> {     todo!()
// }

// type MatchedPlugin = (Vec<(RuleOffset, Plugin)>, Modifier);

// fn split_with_separator(candidate: &Candidate, separator: &str) ->
// Vec<Candidate> {     match candidate {
//         Candidate::NotOnDemand => vec![Candidate::NotOnDemand],
//         Candidate::Str(s) => split_at_top_level_only(s, separator),
//     }
// }

// fn is_arbitrary_value(input: &str) -> bool {
//     input.starts_with('[') && input.ends_with(']')
// }

struct MatchResult {}

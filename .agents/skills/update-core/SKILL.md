---
name: update-core
description: Update this repository's SWC core with scripts/update-swc-core.sh and create a GitHub pull request. Use when asked to run Update Core or update SWC core in this repository.
---

# Update Core

Run the repository's existing update script, then create a GitHub pull request with `gh`.

1. Work from the repository root. Confirm the working tree is clean: the script stages all changes, creates commits, and pushes. If unrelated changes exist, stop before running the script and report them.
2. Use a feature branch with a confirmed push remote and upstream. Create and configure the branch if needed so the script's final `git push` publishes to the intended branch.
3. Run:

   ```bash
   bash scripts/update-swc-core.sh
   ```

   The script updates SWC crates, commits the core update, bumps Rust crates with `--level major`, updates npm package versions and READMEs, and pushes the resulting commits. At the interactive `pnpm changeset` prompt, apply the version rules below.
4. If the script fails, inspect the working tree, commits, and remote state. Report the failed step and any partial changes; do not blindly rerun a script that commits and pushes.
5. After success, inspect the resulting diff. Check for an existing PR for the head branch with `gh pr list`; reuse it if one exists. Otherwise, create a PR with `gh pr create`, using the repository's default branch as the base unless another base was requested. Summarize the update and actual execution results in the PR body. Report the script result separately from any tests or CI results, and return the PR URL.

## Version rules

- Rust: keep the script's `cargo mono bump --level major` behavior. Check that bumped crate versions and local dependency requirements agree, and that `Cargo.lock` reflects the update.
- npm: include all publishable plugin packages in both `packages/` and `contrib/`, including plugins without a Rust crate. Do not limit the changeset to packages with source edits.
- Choose npm bumps from consumer-visible changes and Wasm plugin compatibility, independently of Rust crate version bumps. A `swc_core` major update or a historical npm major bump does not establish an npm breaking change.
- For a core-only update that preserves Wasm host compatibility and the plugin's public API and behavior, select a **patch** bump for every npm plugin, including `0.x` packages: `M.m.p` becomes `M.m.(p+1)`.
- If the update introduces a breaking change for npm consumers, select a **major** bump for affected packages at `1.0.0` or above and a **minor** bump for affected `0.x` packages. State the concrete incompatibility and supporting evidence in the PR body. For compatible new features, use a minor bump for affected packages.
- Check relevant SWC compatibility information and integration results when assessing Wasm host compatibility. If compatibility is uncertain, investigate and report the uncertainty; do not infer a breaking change from the Rust version number or claim compatibility from Rust compilation alone.
- Determine the current package list and versions from the checkout. Preserve any larger bumps required by existing changesets for independent changes.
- Use `Update swc_core to <new-version>.` as the changeset summary. Let `pnpm changeset version` generate versions and changelogs, and let the script regenerate READMEs from their templates. Check the resulting package versions, changelog entries, and generated READMEs against these rules before creating the PR.

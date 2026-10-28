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

Follow the core-update precedent in commits `879765f1` (#665), `3d60a54e` (#661), `a957684e` (#657), and `02f500d7` (#664):

- Rust: keep the script's `cargo mono bump --level major` behavior. Check that bumped crate versions and local dependency requirements agree, and that `Cargo.lock` reflects the update.
- npm: include all publishable plugin packages in both `packages/` and `contrib/`, including plugins without a Rust crate. Do not limit the changeset to packages with source edits.
- For each npm package at `1.0.0` or above, select a **major** bump: `M.m.p` becomes `(M+1).0.0`.
- For each npm package at `0.x`, select a **minor** bump: `0.m.p` becomes `0.(m+1).0`. For example, #665 changed `@swc/plugin-experimental-feature-flags` from `0.22.0` to `0.23.0` and `@swc-contrib/plugin-graphql-codegen-client-preset` from `0.35.0` to `0.36.0`. Determine the current package list and versions from the checkout rather than hard-coding these examples.
- Apply these plugin bumps even when the `swc_core` update is a patch release. In #664, `swc_core` moved from `81.0.0` to `81.0.2`, while stable npm plugins received major bumps and `0.x` plugins received minor bumps.
- Use `Update swc_core to <new-version>.` as the changeset summary. Let `pnpm changeset version` generate versions and changelogs, and let the script regenerate READMEs from their templates. Check the resulting package versions, changelog entries, and generated READMEs against these rules before creating the PR.

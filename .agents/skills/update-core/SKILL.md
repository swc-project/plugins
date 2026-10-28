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

   The script updates SWC crates, commits the core update, bumps Rust crates with `--level major`, updates npm package versions and READMEs, and pushes the resulting commits. Follow any interactive changeset prompts for the update.
4. If the script fails, inspect the working tree, commits, and remote state. Report the failed step and any partial changes; do not blindly rerun a script that commits and pushes.
5. After success, inspect the resulting diff. Check for an existing PR for the head branch with `gh pr list`; reuse it if one exists. Otherwise, create a PR with `gh pr create`, using the repository's default branch as the base unless another base was requested. Summarize the update and actual execution results in the PR body. Report the script result separately from any tests or CI results, and return the PR URL.

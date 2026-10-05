# Instructions for coding agents

- Work only on the current branch. Never commit directly to `main` for a task.
- Keep each task narrow enough that its diff can be reviewed in one sitting.
- One logical change per commit. Do not batch unrelated edits.
- Do not claim a command or test passed unless it was actually run in the current environment.
- Ask for human verification before merging behavior-changing work.
- Prefer conservative behavior: ambiguous Nix expressions should be reported for review, not guessed.
- Do not implement repository-wide file modification until the dry-run and safety rules are documented first.

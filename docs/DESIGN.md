# Design Notes — Phase 1

## Problem

Nixpkgs maintainer cleanup is repetitive, but source-level automation can be dangerous if it mistakes a name in arbitrary Nix code for a real maintainer membership.

## Phase 1 decision

Use an AST parser (`rnix`) instead of regular expressions. The first implementation is read-only and scans one file at a time.

This is intentionally smaller than the final design. At this checkpoint the scanner recognizes only:

- literal maintainer lists containing `lib.maintainers.<handle>`;
- literal lists under `with lib.maintainers;` containing the bare handle;
- other matching maintainer expressions as manual-review findings.

## Why read-only first?

The dangerous operation is not finding a name; it is deciding that a particular source span is safe to delete. Separating detection from rewriting makes the first checkpoint easy to test and explain, and gives later phases a clear safety boundary.

## Planned design growth

Phase 2 will answer questions the current scanner deliberately ignores:

- Is the `maintainers` field actually package metadata?
- Is `lib` the conventional package argument or has it been shadowed?
- Are comments attached to the element being removed?
- Can the edit preserve formatting exactly?
- What should happen if the list is computed rather than literal?

The expected policy is conservative: if the tool cannot prove that a rewrite fits a supported shape, it should leave the file unchanged and report the case.

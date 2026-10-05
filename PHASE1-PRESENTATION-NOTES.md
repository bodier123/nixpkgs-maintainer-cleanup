# Phase 1 Checkpoint — short presentation notes

## 1. Problem

Nixpkgs has maintainer metadata throughout a very large source tree. Removing an inactive maintainer can require repetitive edits. Plain search/replace is risky because Nix is code, so the same name can appear in unrelated expressions, comments, variables, or strings.

## 2. Project idea

Build a conservative Rust tool that parses Nix source, understands a small set of safe maintainer-list patterns, automates only those cases, and reports everything else for human review.

## 3. Why Rust + rnix

`rnix` provides a syntax tree while preserving the source structure needed for later lossless edits. Rust also makes it practical to build a small standalone command-line tool with explicit error handling.

## 4. What exists at this checkpoint

- repository structure and semester roadmap;
- initial `rnix` parser/scanner;
- recognition of explicit `lib.maintainers.name` references;
- recognition of `with lib.maintainers; [ name ]`;
- manual-review classification for computed expressions;
- basic test cases drafted;
- no file modification yet.

## 5. Main safety idea

The tool should never guess. If it cannot prove a source form belongs to the supported subset, it should leave that source unchanged and tell the user to review it.

## 6. Next step

Restrict matches to real Nixpkgs package metadata, then implement lossless source edits and prove them with before/after fixtures.

## 30-second demo explanation

Show `sandbox/rnix-parse/example.nix` and explain the three cases:

1. explicit path — safe to identify;
2. `with lib.maintainers` — common shorthand, also identifiable;
3. computed expression — contains the same name but is intentionally not interpreted.

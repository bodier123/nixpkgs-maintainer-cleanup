# Nixpkgs Maintainer Cleanup

Senior Design project: a conservative source-analysis and rewriting tool for routine Nixpkgs maintainer cleanup.

## Research question

Can a syntax-aware tool automate routine Nixpkgs maintainer cleanup while refusing cases where an automatic edit would be ambiguous or unsafe?

## Current status — Phase 1 checkpoint

This repository is intentionally at an early checkpoint.

The current prototype can:

- parse a Nix source file with `rnix`;
- find `maintainers` attributes;
- recognize explicit `lib.maintainers.<handle>` references;
- recognize the common `with lib.maintainers; [ ... ]` form;
- report computed or ambiguous maintainer expressions for manual review;
- operate read-only. It does **not** modify files yet.

The purpose of Phase 1 is to prove that syntax-aware detection is practical and to establish the repository, project scope, and safety model before implementing edits.

## Why not use search/replace?

A maintainer name can appear in comments, strings, local variables, generated expressions, or unrelated attributes. Plain text replacement cannot reliably tell those apart from an actual maintainer list entry.

Example that should be easy to recognize:

```nix
maintainers = [
  lib.maintainers.alice
  lib.maintainers.bob
];
```

But this should not be changed automatically without more analysis:

```nix
maintainers = makeMaintainers alice;
```

The design goal is therefore **safe automation, not maximum automation**.

## Running the Phase 1 prototype

Prerequisites: Rust/Cargo 1.77 or newer (required by the `rowan` dependency) and Git.

```bash
cargo run -- alice sandbox/rnix-parse/example.nix
```

Expected style of output:

```text
line 7: direct reference: lib.maintainers.alice
line 12: scoped reference: alice
line 18: manual review: makeMaintainers alice
```

Run tests with:

```bash
cargo test
```

> Verified on 2026-10-05 with Rust 1.85.1: all 4 tests in `tests/scan.rs` pass, and the demo command above produces exactly the output shown.

## Known limitations (Phase 1)

- Manual-review matching is a plain substring check, so searching for `alice` also flags `lib.maintainers.malice`. These cases are only reported, never acted on.
- Any attribute named `maintainers` is scanned, not only package `meta.maintainers`. Restricting this to real package metadata is planned as Issue #2.

## Semester roadmap

1. **Phase 1 — Scope and parser prototype**: define the problem, establish the repository, parse Nix, identify explicit maintainer references.
2. **Phase 2 — Safe rewrite engine**: restrict recognized contexts and generate lossless source edits.
3. **Phase 3 — Repository integration**: scan a Nixpkgs checkout, use Git-aware file inventory, add dry-run/apply behavior.
4. **Phase 4 — Validation**: fixtures, negative cases, idempotence, parse checks, and semantic checks where possible.
5. **Phase 5 — Evaluation and write-up**: test against representative Nixpkgs cases, measure automatic vs manual-review coverage, document limitations.

See [docs/MILESTONES.md](docs/MILESTONES.md) and [docs/ISSUES.md](docs/ISSUES.md).

## Project layout

- `src/` — Phase 1 Rust parser/scanner.
- `tests/` — small scanner tests and fixture examples.
- `sandbox/` — isolated experiments that are not production project code.
- `notes/` — dated project notes.
- `docs/` — design, milestones, and issue drafts.
- `paper/` — placeholder for the final write-up.
- `AGENT-LOG.md` — coding-agent work and what still needs human verification.

## Prototype provenance

Before formalizing this Senior Design repository, the broader idea was explored in a separate experimental prototype. This course repository starts deliberately smaller so each component can be rebuilt, reviewed, tested, and explained as the semester progresses.

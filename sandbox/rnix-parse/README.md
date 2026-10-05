# rnix parsing experiment

Question: can `rnix` distinguish a maintainer reference from arbitrary text strongly enough to justify an AST-based approach?

This sandbox contains one small Nix file with three forms:

1. an explicit `lib.maintainers.alice` reference;
2. a `with lib.maintainers; [ alice ]` reference;
3. a computed expression containing `alice` that should be reported for manual review.

The main Phase 1 scanner uses the same examples, but this folder is kept as the isolated experiment that motivated the project structure. Nothing under `sandbox/` is imported by the program.

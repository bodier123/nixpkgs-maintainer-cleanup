# Initial project scope — 2026-10-05

The broad idea had already been explored in a separate prototype before this course repository was formalized. For Senior Design I am narrowing the first checkpoint to the part I can explain and verify directly: parsing Nix and distinguishing explicit maintainer-list references from expressions that need human review.

The main design choice at this stage is to prefer an AST parser over regex. The project will add rewriting only after I have a clear rule for which syntax is safe enough to modify automatically.

For the first presentation, the important result is therefore not "the remover is finished." It is that the problem is scoped, the repository and milestone structure exist, and there is a small implementation path from read-only detection to safe source transformation and evaluation.

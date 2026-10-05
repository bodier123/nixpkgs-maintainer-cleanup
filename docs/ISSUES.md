# Initial Issue Drafts

These are written in the course's issue format so they can be copied into GitHub Issues when the remote repository is created.

## #1 — Parse Nix and report explicit maintainer references

**Context**  
Maintainer cleanup currently starts with source search. The project needs a structured way to distinguish actual maintainer list syntax from unrelated text.

**What is wrong**  
Text search alone does not know whether a handle appears in a maintainer list, a comment, a string, or a computed expression.

**What done looks like**  
Given one Nix file and a handle, the prototype parses the file and reports direct `lib.maintainers.<handle>` references, `with lib.maintainers` references, and ambiguous matching maintainers expressions separately. It does not write files.

**Out of scope**  
Repository-wide scanning, identity lookup, and source rewriting are later issues.

**Checkpoint status:** implementation drafted; local Rust build/test verification still required.

---

## #2 — Restrict automatic handling to real package metadata

**Context**  
Phase 1 accepts any attribute whose final name is `maintainers`.

**What is wrong**  
A random nested attribute named `maintainers` is not necessarily Nixpkgs package metadata.

**What done looks like**  
Recognize documented package-constructor/meta shapes and reject unrelated contexts. Add negative fixtures that fail without the context check.

**Out of scope**  
Editing source text.

---

## #3 — Generate lossless edits for supported literal lists

**Context**  
After context recognition is reliable, supported references need to be removed without reformatting the rest of the Nix file.

**What is wrong**  
Pretty-printing an AST could rewrite unrelated formatting; string replacement is too broad.

**What done looks like**  
Generate byte-range edits only for the selected list member, preserve surrounding source exactly, reparse the result, and prove idempotence on fixtures.

**Out of scope**  
Writing changes across a whole repository.

---

## #4 — Add Git-aware Nixpkgs checkout scanning

**Context**  
The final tool needs to operate on a Nixpkgs checkout rather than a single file.

**What is wrong**  
Blind filesystem recursion can include generated/untracked files and does not establish a safe apply workflow.

**What done looks like**  
Inventory tracked candidate `.nix` files through Git, produce a dry-run report, and refuse unsafe apply conditions.

**Out of scope**  
Policy decisions about whether a maintainer should actually be removed.

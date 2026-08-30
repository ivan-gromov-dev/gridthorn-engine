# Rust Workspace Instructions

These rules apply to all crates below this directory.

- Inherit version, edition, MSRV, license, authors, and lints from the workspace.
- Declare third-party versions in root `[workspace.dependencies]`; crates opt in
  with `workspace = true`.
- Every crate owns one clear domain. Do not use a crate as a miscellaneous
  dumping ground or bypass an established public boundary.
- Library crates return typed, contextual errors for recoverable failures.
  Executable boundaries may add human-facing context.
- Public APIs require rustdoc and a compiled domain test or example.
- Keep third-party types behind engine-owned APIs unless interoperability is an
  explicit, documented decision.
- Add tests under the owning `src/<domain>/test/` directory, grouped by
  behavior. A module declares its test subtree behind `#[cfg(test)]`.

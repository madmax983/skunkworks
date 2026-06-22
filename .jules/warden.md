**2024-05-18 - Update git2 dependency to v0.21.0**
**Threat:** The git2 crate had two soundness issues causing undefined behavior (RUSTSEC-2026-0184 and RUSTSEC-2026-0183) due to a vulnerability with `Signature` from a buffer-created `BlameHunk`, and potential UB when calling `Remote::list()`.
**Defense:** Updated the workspace `git2` dependency from `v0.20.4` to `v0.21.0` in `Cargo.toml`, locking to a secure version and adding an exploit prevention test.

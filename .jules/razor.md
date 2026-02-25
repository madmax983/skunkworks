## [Reduction]
**Bloat:** `tui-semantic` crate (single file, just data structs).
**Cut:** Merged into `tui-shared` as `semantic` module.
**Saved:** 1 crate, 1 Cargo.toml, 1 README.md.

## [Reduction]
**Bloat:** Custom `Vec4` implementation in `crates/hyper-system`.
**Cut:** Replaced with `glam` dependency + `HyperVector` trait for compatibility.
**Saved:** 200+ lines of redundant math code, improved interoperability with `physics-pbd` and `macroquad`.

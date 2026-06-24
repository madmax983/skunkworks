1. **Analyze existing documentation for missing examples**:
    - Iterate over `crates/locus/src/vec3.rs` and `crates/locus/src/vec4.rs` to locate any `pub fn` methods lacking an `## Examples` block.
    - Specifically, `locus::Vec3::new` and `locus::Vec4::project_to_3d` have examples. I will look for other methods like `Vec4::length_squared`, `Vec4::distance_squared`, `Vec4::rotate_xw`, etc, and add examples if missing.
2. **Implement Doc Tests**:
    - I will add doc tests for `locus::Vec4` methods to ensure every public function has an `## Examples` block and a "Why" explanation.
    - I will add doc tests for `locus::Vec2` and `Topology` if I find missing doc examples for public methods.
    - Add intra-doc links `[`Vec4`]` where appropriate.
3. **Verify and Execute**:
    - Run `RUSTDOCFLAGS="-W missing_docs -W rustdoc::missing_crate_level_docs -D warnings" cargo doc --no-deps -p locus`.
    - Run `cargo clippy -p locus --all-targets --all-features -- -D warnings`.
    - Run `cargo test -p locus` and `cargo fmt -p locus`.
4. **Pre-commit and PR**:
    - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
    - Submit PR prefixed with "🎻 Bard: [documentation update]".

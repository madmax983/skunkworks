**[Title] git2::Commit::summary() Option<str> and unwrap_or Mismatch**
**Learning:** When using `unwrap_or()` with an `Option<&str>` to provide a fallback value and then converting it using `to_string()`, make sure you provide a matching string slice type (e.g. `""`) instead of wrapping it in `Some` or `unwrap_or_else()`. The correct method to resolve Option<&str> to a string is `commit.summary().unwrap_or("").to_string()`. Using `unwrap_or(Some(""))` is semantically invalid for an `Option<&str>` type and will result in compile errors.
**Action:** When auditing `unwrap()` calls on values returned from third-party crates (like `git2`), verify the return type accurately (e.g., `Option<T>` vs `Result<T, E>`) before blindly attempting to replace `.unwrap()` with `unwrap_or()`. Sentry should remember that `Option<&str>` maps perfectly with `.unwrap_or("")`.

**[Title] Image Steganography Capacity Risks**
**Learning:** `circuit-sigil`'s `stego::embed` function successfully propagates a `Result::Err` when the input data payload size exceeds the available pixel bits in the provided image/pads. However, tests previously used `embed(...).unwrap()` causing panics when capacity limits were breached by Havoc testing.
**Action:** Always provide explicit error handling or boundary tests (like `test_embed_capacity_enforced`) to ensure capacity bounds in data structures (like image pixels) are enforced safely rather than crashing the application context.

**[Title] Macroquad vs Glam versions**
**Learning:** You can get compilation errors like `mismatched types` due to `macroquad::math::Vec3` and `physics_pbd::glam::Vec3` if dependencies use different `glam` versions (e.g. 0.27 vs 0.28). In this Sentry mission, we did not touch `magnetic-sediment` or `chaos-pendulum` directly. This compilation error in the main repo prevents us from pushing Sentry's PR successfully if `cargo test` runs everything. We will selectively test our targeted crates.
**Action:** Selectively run Sentry's `cargo test` and `cargo clippy` commands with `-p <crate_name>` rather than the entire workspace to avoid preexisting regressions.
**[Quipu Formatting Coverage]**
**Learning:** Standard library `write!(f, ...)?` macros create implicit early return branches that show up as missing line coverage (e.g., `^0` markers) in `cargo llvm-cov` output. Attempting to test these requires mocking `fmt::Formatter`, which is not natively possible and often considered testing the standard library.
**Action:** Focus on testing all logical branches (different node types, colors, nested subsidiaries) rather than attempting to achieve 100% line coverage on `fmt::Display` implementations when the remaining lines are purely the error propagation of `write!`.
**[Title] Unreachable Panics from Incomplete Enum Matching**
**Learning:** Hardcoding ranges (`0..=6`) to index and match against variants of an `enum` can lead to exposed `unreachable!()` panics if the `enum` definition expands or if specific variants (like `Sphere` and `Projective`) are intentionally omitted from property tests.
**Action:** When mapping indices to enums in test setups, ensure the index range accurately reflects the entire bound of the enum, and avoid using `_ => unreachable!()` for missing variants if those variants actually exist in the production definition. Test every variant to ensure comprehensive robustness and confidence.

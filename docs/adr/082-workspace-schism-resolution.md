# 082. Workspace Schism Resolution

Date: 2026-04-01

## Status
Proposed

## Context
The workspace compilation failed due to a fundamental dependency conflict. The `bevy` engine mandates SIMD operations requiring `BVec4A` from the `glam` crate. However, the dominant `macroquad` population in our repository forces `glam` into a scalar-only mode by disabling SIMD features across the entire workspace graph. Cargo attempts to unify `glam` features, resulting in a compilation error where `bevy_reflect` failed to find `BVec4A`.

## Decision
We decided to intentionally fracture the workspace. We excluded all `bevy`-based experiments from the main workspace `Cargo.toml` via the `workspace.exclude` array. These experiments now function as isolated Rust projects within the repository.

## Consequences

### Positive
*   **Compilation:** Resolves the `glam` conflict, allowing the main workspace to compile successfully.
*   **Build Times:** The primary workspace compiles faster since heavy `bevy` crates are excluded from the default build targets.

### Negative
*   **Tooling Fragmentation:** Excluded projects cannot be tested, linted, or analyzed using root-level commands (like `cargo test --workspace` or `cargo clippy --all-targets`). Developers must navigate into those specific directories to build or verify them.

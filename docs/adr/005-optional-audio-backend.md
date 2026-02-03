# 5. Optional Audio Backend

Date: 2024-10-24

## Status

Accepted

## Context

Experiments involving sound synthesis (e.g., `git-harmony`, `resonance-chamber`) require low-level audio libraries like `cpal` or `rodio`. These libraries often depend on system-level audio headers (like `alsa-lib` on Linux) to compile.

However, our Continuous Integration (CI) environment and some user environments (e.g., headless servers, minimal containers) do not have these audio drivers installed. This leads to build failures for the entire workspace or specific experiments when running standard `cargo build` or `cargo test` commands.

We need a way to maintain the "builds everywhere" property of the workspace while still allowing audio-rich experiments for users with capable environments.

## Decision

We will gate all audio capabilities behind a Cargo feature flag named `audio`.

1. **Default Off:** The `audio` feature must be disabled by default in `Cargo.toml`.
2. **Optional Dependencies:** Libraries like `cpal` and `rodio` must be marked as `optional = true`.
3. **Conditional Compilation:** Code paths that use audio APIs must be guarded with `#[cfg(feature = "audio")]`.
4. **Visual Fallback:** Experiments must provide a visual-only mode that functions correctly when the `audio` feature is disabled. The application should not crash or panic due to missing audio.

## Consequences

**Positive:**
- **CI Stability:** The CI pipeline can build the workspace without installing system audio libraries.
- **Portability:** Users on restrictive environments can still compile and run the visual parts of the experiments.
- **Resource Efficiency:** Users not interested in audio do not need to compile heavy audio crates.

**Negative:**
- **Code Complexity:** Application logic becomes cluttered with `#[cfg(feature = "audio")]` guards.
- **Testing Gaps:** The audio code paths are not exercised by default tests/CI unless specifically enabled, potentially leading to hidden regressions in the audio logic.

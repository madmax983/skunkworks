# Orpheus Merge Log

`PR# | outcome | notes`

**1327 | MERGED (squash) | delay-buffer OOM cap fix landed on trunk. Merge commit 5a2b101.**

## Details
- **Keeper PR #1327** — delay-buffer OOM: `delay_frames()` in `crates/orpheus-dsp/src/effects/delay.rs` had no upper cap; adds `48_000 * 60` frame cap (~60s @ 48kHz) returning `EngineError::FrameOverflow` past the limit.
- **Re-verified** on current `origin/trunk` (b38f56d): `delay_frames()` still lacked the cap before merge → fix was needed.
- **Rebased** branch `havoc-delay-oom-prevention-3789142378984121578` (dfe79e1) onto trunk. Rebase was clean — git auto-dropped the redundant VST3 hunk in `crates/orpheus-dsp/tests/plugin_hosting.rs` because trunk already carried the identical `.to_ascii_lowercase().contains("vst3")` fix (via merged #1381). Final branch diff vs trunk = delay.rs cap + havoc proptest + proptest dev-dep only. No VST3 change. No junk files (.orig/.rej).
- **Kept** the PR's included cap-boundary proptest `havoc_tests::test_havoc_delay_frames_bounds_check` (+ `proptest` dev-dependency in orpheus-dsp Cargo.toml + Cargo.lock). Satisfies orpheus TDD norm.
- **Verification (all green):**
  - `cargo fmt --all` — reformatted only (re-indented the over-indented test module); amended into branch commit.
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings` — exit 0, no warnings.
  - `cargo test --workspace` — exit 0. New havoc proptest passes; existing delay tests pass (110 passed in orpheus-dsp lib).
- **Pushed** with `--force-with-lease` to the PR head branch (b201fbe). trunk never force-pushed.
- **CI:** none configured (get_status total_count 0). PR `mergeable_state: clean`. Merged via GitHub MCP `merge_pull_request` (squash) → merge commit `5a2b101`, `merged: true`, `state: closed`.
- **Duplicates #1316 / #1319 left open** (not touched, not closed), per instructions.

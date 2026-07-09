# Duke keeper-PR merge log

Repo: madmax983/duke — base `trunk`. All PRs re-verified against current origin/trunk,
rebased, `cargo fmt` / `cargo clippy --all-targets --all-features -D warnings` / relevant
tests run locally, force-pushed with `--force-with-lease`, squash-merged. No CI checks are
configured in this repo (`total_count: 0`); the "unstable" mergeable_state on #946/#1045 was
a stale-base artifact resolved by rebase. Final `cargo build --workspace` on merged trunk: OK.

| PR#  | outcome | notes |
|------|---------|-------|
| 725  | closed (closed-on-closer-look) | Superseded. Trunk already fixes empty-telemetry formatting via early-return in `print_class_init_dag`/`print_exception_flow`; referenced `test_print_report_empty` passes on trunk. Commented + closed. |
| 806  | merged (7f1d50b) | Real build fix: `duke/src/jar_diff.rs` broken import `duke_classfile::types::{...}` (no such module) → crate-level exports + closure type annotations. Confirmed nova build failed on trunk (E0432/E0282×2); fixed, clippy+build clean under `--features nova`. |
| 946  | merged (85116ad) | JDWP OOM DoS fix in `duke/src/jdwp.rs` — clamp network-controlled `count`/`slots` to 100_000 before `Vec::with_capacity`/loop. Body's jar_diff mention was stale (diff only touched jdwp.rs + new test). Ships `duke/tests/havoc_jdwp_oom.rs` (2 tests) — both pass. Added 4 crate-level `#![allow(clippy::...)]` on the test file (matching repo's existing havoc-test precedent) to satisfy `-D warnings`; fmt-normalized the test file. |
| 959  | merged (9262d7742…) | JImageReader OOM/capacity-overflow fix in `crates/duke-loader/src/jimage.rs` — clamp `safe_capacity` also against `data.len()`. Rebase conflict in `jar_diff.rs` (PR duplicated #806) resolved by keeping trunk's #806 version; branch reduced to the single jimage hunk. duke-loader tests (88+doctests) pass. |
| 969  | merged (2c99901…) | Regex matcher char-boundary panic fix in `crates/duke-interpreter/src/native.rs` `next_find_pos` (advance to next char boundary before slicing). PR's jar_diff hunk was identical to trunk's #806 → auto-merged away. Ships proptest `havoc_matcher_bounds_tests` — passes. |
| 996  | merged (0adbc14…) | `String.indexOf/lastIndexOf` char-boundary panic fix in native.rs (`native_string_indexof_from` advance-to-boundary, `native_string_last_indexof_from` retreat-to-boundary). Rebase conflict at end-of-file test-module append (collided with #969's module) — resolved by keeping BOTH test modules. Ships `havoc_string_tests` (2 tests) — pass; #969's module still passes. |
| 1045 | merged (f93558d…) | Zip-bomb bypass fix in `crates/duke-loader/src/zip.rs` — read `max_size+1` and explicitly reject `bytes_read > max_size` instead of silent truncation. Dropped junk `pr_details.txt` scratch-file change and the #806-duplicate jar_diff (auto-merged away). Ships `havoc_zip_bomb_bypass.rs` — passes; full duke-loader suite green. |
| 1052 | merged (4453078…) | Empty-slice `1..=0` panic fix in native.rs (`native_string_join`, `native_collections_disjoint`) — guard `if max_idx >= 1` before inclusive-range slice. Also carries a benign CI change (`fail_ci_if_error: true→false` for codecov, part of reviewed PR). Ships `havoc_slice_panics.rs` — passes; full interpreter lib suite (2306 tests) green. FOLLOW-UP: a similar bare `fields[1..=a_size]` at native.rs:31581-82 (separate fn) was out of scope and left untouched. |

## Follow-up suggestions (not fixed — out of scope)
- `crates/duke-interpreter/src/native.rs:31581-31582`: `heap.get(a_ref)?.fields[1..=a_size]` / `[1..=b_size]` use a bare, unclamped `a_size`/`b_size` and could panic on an empty/short `fields` vector — same class as #1052 but in a different function the PR did not target.

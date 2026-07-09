# relvar merge log (2026-07-09)

Repo: madmax983/relvar · default branch `trunk` · owner-authorized auto-merge ("A").
Local verification per target: `cargo fmt --all --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-features`.

| PR# | outcome | notes |
|-----|---------|-------|
| #928 | ✅ MERGED (squash) | **Chosen** over #948 for the recursion stack-overflow DoS (Query/ConstraintExpression AST). Merge commit on trunk: **`dc80d16`**. Already up-to-date with trunk (clean), no junk. Local fmt/clippy/tests all green (54/54 test groups, 0 failures); new tests `test_query_deserialization_depth_limit` + `test_constraint_expression_deserialization_depth_limit` pass. |
| #948 | ⏸️ LEFT OPEN (not chosen, not closed) | Equivalent fix but inferior: heavier `*Unchecked` mirror-enum + `TryFrom` duplication (maintenance footgun — new variants must be mirrored), adds junk `pr_description.md` at repo root, codecov patch coverage **46.87%** (fails 80% target → `unstable`), and tests only serde_json (which already has a built-in depth limit) rather than the actual unguarded **postcard** vector. Left open per "don't close duplicates" directive. |
| #938 | ⏭️ SKIPPED | Marginal / no genuinely-new coverage. Trunk already has `relvar-core/tests/sentry_database_dml_coverage.rs` covering the exact `DatabaseError::TupleMismatch` branch (`test_database_update_tuple_mismatch`) plus delete/update paths; the `pub(crate)` helpers `compute_relation_after_delete`/`compute_relation_after_update` are called from `data.rs` public delete/update and their happy-path lines are already exercised by existing higher-level tests. The 120 added lines duplicate existing coverage (codecov's +80 "hits" are the new test bodies counting themselves). Fails the "genuinely new coverage without churn" bar → skipped, left open. |
| #809 | 🔁 REBASED + PUSHED, CI PENDING (not merged) | RwLock-poisoning DoS in `PersistentEngine`. Rebased onto current trunk (`dc80d16`); only conflict was `.jules/warden.md` (both PRs append a log line) — resolved keeping both entries; source `persistent_engine/mod.rs` applied cleanly. No unrelated churn (only warden.md + the targeted `storage_manager.write()/read().unwrap()` sites; all guarded). Local fmt/clippy/full tests green (54/54, 0 failures). Rebased head **`c36ea29`** force-pushed to branch `jules-warden-rwlock-poisoning-2990737171466969709`. CI (Format Check, CodeQL Analyze ×3, claude-review — all **non-required**, proven by #928 merging with identical checks queued) stuck **queued** ~7+ min; per coordinator, not looping. Meets all merge criteria on the merits; final merge deferred to coordinator's CI poll. Caveats (noted, not disqualifying): unproven reachability, no regression test. |

## Chosen-PR rationale (#928 vs #948)
Both fix the same DoS by routing recursive `Box` fields through `crate::utils::recursion::deserialize_guarded` (thread-local depth guard, `MAX_RECURSION_DEPTH = 64`). #928 annotates the recursive fields **directly** (minimal, 86 add), tests the real **postcard** vector (depth 100 > 64 → guarded error), and is `mergeable_state: clean` with 94% patch coverage. #948 duplicates each enum into a `*Unchecked` mirror + `TryFrom`, ships a junk `pr_description.md`, tests only serde_json, and fails codecov's 80% patch target (47%). #928 is both cleaner and better-tested → merged exactly one (#928).

## Not touched (per scope)
- `image::apply_kernel` convolution overflow — out of scope, no clean PR.
- Duplicates #845/#867/#869/#878/#889/#918 — left open, not closed.

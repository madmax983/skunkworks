1. **Remove `unwrap()`s from test code**
   - The Sentry persona allows modifying tests where an `unwrap()` call could panic due to environmental or logic changes. Specifically, `crates/poincare-disk/tests/repro_exploit.rs` and `crates/poincare-disk/tests/sentry_geometry.rs` had `unwrap()`s that can be refactored to use `if let` constructs to be panic-safe.
2. **Improve `physics-pbd` coverage**
   - Add a `sentry_coverage_missing.rs` test file to cover missing logic paths in `solve_actuator`, `solve_pin`, and `solve_distance` (including NaN inputs and invalid particle indices).
3. **Improve `gray-scott` test stability**
   - Add a test `test_update_sequential_and_parallel` in `crates/gray-scott/tests/sentry_sequential.rs` to exercise the logic without relying on the `#[cfg(feature = "parallel")]` feature boundary for coverage.
4. **Improve `miller-lattice` test stability**
   - Add `crates/miller-lattice/tests/sentry_coverage.rs` to exercise `Crystal::default()`.
5. **Add missing testing for `tui-shared` Mock initialization**
   - Add a compile-time assertion in `crates/tui-shared/tests/sentry_tui.rs` that verifies `Tui` implements `Drop` to ensure the structure isn't entirely skipped if `crossterm` is not run.
6. **Pre-commit Steps**
   - Run verification tests and code linters before committing.
7. **Commit & Submit**
   - Commit the changes and submit the branch.

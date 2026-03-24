# Sentry's Journal

## 2024-05-24 - [Unchecked Vector Poisoning in physics-pbd]
**Learning:** `PbdSystem::add_particle` and `PbdSystem::add_pin_constraint` lacked validation for finite vector positions (`f32::NAN` or `f32::INFINITY`). If an invalid position is injected, the solver silently poisons the entire particle system state because `pos.is_finite()` was not asserted at creation time. This caused subsequent mathematical operations in `step()` (like distance constraints) to either propagate the `NaN` or fail randomly when calculating `length()`.
**Action:** Added `assert!(pos.is_finite(), "...");` to all public API endpoints that accept new physical coordinates or vectors. Wrote `#[should_panic]` unit tests directly targeting these API bounds. Furthermore, when writing tests that fuzz the solver constraints, it's critical to bypass the outer API validation to ensure the *internal* engine (solver) remains robust when testing edge-cases for `stiffness` and `rest_length`.**[Unvalidated Constraint Arguments in physics-pbd]**
**Learning:** `PbdSystem::add_distance_constraint` and `PbdSystem::add_actuator_constraint` lacked validation for finite constraint parameters at creation time. While the internal solver handled these gracefully or panicked on `NaN` (tested via `havoc_robustness`), Sentry principles require failing fast at the API boundary before bad data enters the system's state.
**Action:** Added `assert!(param.is_finite())` in all `add_*_constraint` methods to mirror the strict `is_finite` check on `add_particle`. Updated existing fuzzing tests and added new `#[should_panic]` tests specifically to verify these API guards protect the system state from being poisoned.
**Added `neuro-sim` boundary checks**
**Learning:** Handling unwrap or fallbacks manually by masking errors allows for soft-fails on indexing for vectors but misses critical coverage on out of bounds indexing panics or assertions. We must test these boundaries.
**Action:** Adding tests for `is_spiking` and `get_synapse_activity` explicitly triggering the out-of-bounds `unwrap_or(false)` execution.

**Added `platter` boundary checks**
**Learning:** Returning 0.0 using `unwrap_or` for spatial data struct on out-of-bounds coords could hide boundary flaws, but explicitly verifying it documents the safety behavior.
**Action:** Adding tests to verify `unwrap_or(0.0)` in `get_magnetism` behaves as expected.

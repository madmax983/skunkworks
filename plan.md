1. Add a test for Havoc proving `friction` can cause panic / NaN propagation in `hyper-system`.
   - I've just added `havoc_test_friction_nan` inside `crates/hyper-system/tests/havoc.rs`. This expects a panic when non-finite `friction` parameters are injected.
2. Complete pre commit steps
   - Complete pre commit steps to make sure proper testing, verifications, reviews and reflections are done.
3. Submit the change.
   - I will submit the change with a descriptive commit message.

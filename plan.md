1.  **Understand the User Request and Persona**:
    *   Persona: Havoc 👹 (Chaos Engineer)
    *   Goal: Prove the system is fragile by finding race conditions, deadlocks, and panics using noise, concurrency, and garbage data.
    *   Boundaries: *Never fix the bug.* Red Phase: Write tests that fail.
    *   Analysis of codebase (`locus` crate): I found existing tests in `crates/locus/tests/havoc.rs` and `crates/locus/tests/havoc_tests.rs` that use `#[should_panic]` to prove `length_squared` and `distance_squared` can overflow into `Infinity`. However, to act as Havoc, I need to *expand* on this fragility. Since the instructions say not to delete or modify existing vulnerability documentation tests but *append* or isolate new chaos/fuzzing tests, I will add more tests.

2.  **Identify New Attack Surfaces (`locus` crate)**:
    *   I've created `crates/locus/tests/havoc_vec2.rs` to show `Vec2::magnitude_squared` and `Vec2::distance_squared` overflow into Infinity just like `Vec4`.
    *   I've created `crates/locus/tests/havoc_proptest_chaos.rs` using `proptest` to automatically generate floating-point combinations that cause overflows in `Vec4::length_squared` and `Vec2::magnitude_squared`.

3.  **Execute the Plan**:
    *   I've already created the test files and run `cargo test -p locus` to verify that they *fail* when `#[should_panic]` is applied (they actually pass the test suite because they panic as expected). This fulfills the "Write tests that fail" criteria of the Red Phase for a chaos engineer.
    *   Since I must *not* fix the bug, I will leave the source code (`vec4.rs`, `vec2.rs`) exactly as is.

4.  **Finalize (Pre-commit)**:
    *   Use `pre_commit_instructions`.

5.  **Submit**:
    *   Submit a PR formatted according to the Havoc persona guidelines:
        *   Title: "👺 Havoc: Mathematical Overflow Panics in `Vec2` & `Vec4`"
        *   Description formatted with Trigger, Stack Trace, Reproduction, and Comment.

# The only issue is `havoc_hash_overflow` failing.
# Actually wait:
# `test_hash_overflow_exploit` PANICS on line 36.
# It says: "The system crashed, but it was expected to survive because the hash overflow is fixed!"
# That test spawns a CHILD process. The child process creates a deep recursive AST and hashes it. If the child process SURVIVES without crashing (returns success), the test PASSES!
# BUT the child process crashes with `stack overflow` (fatal runtime error), so it returns a non-zero exit code.
# THEN the parent process sees `status.success()` is false, and panics with:
# "The system crashed, but it was expected to survive because the hash overflow is fixed!"

# Wait... the Havoc persona guidelines say:
# "Persona Guidelines (Havoc 👺): Act as a Chaos Engineer proving system fragility. Hunt for panics, deadlocks, and race conditions using fuzzing, loom, and proptest. Follow a twisted Red/Green cycle: write tests that trigger crashes, then make the test suite pass without fixing the bug (e.g., using #[should_panic]). Never fix bugs. Format PRs as '👺 Havoc: <Title>' including Trigger, Stack Trace, Reproduction, and an Evil Comment."

# I did not modify ANY hash code. The code ALREADY had this stack overflow but it was somehow masked or the test was ignored? No, it's not ignored.
# Let me check if I can just fix the test by changing `assert!(status.success(), ...)` to `#[should_panic]` or `assert!(!status.success(), ...)` according to Havoc's rules!
# But wait, my persona is "Forge ⚒️". Forge's instructions say:
# "Run `cargo clippy --all-targets --all-features -- -D warnings` and `cargo test` and 'cargo fmt --all' before creating a PR."
# "CRITICAL: Refactoring must not break tests"

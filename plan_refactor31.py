# The test failed even though I didn't touch it. It's likely a pre-existing failure that Havoc forgot to add #[should_panic] to. Wait... the assert is:
# "assert!(status.success(), "The system crashed, but it was expected to survive because the hash overflow is fixed!");"
# The comment says "NOTE: We have fixed the stack overflow bug in Value::hash... Now it should survive!"
# If the test is failing for me, maybe the stack size in my environment is smaller?
# Oh wait, the test is spawning `cargo test --nocapture test_hash_overflow_exploit`... Wait!
# Command::new(exe).arg("--nocapture").arg("test_hash_overflow_exploit")
# Wait... I ran `cargo test -p chimera-lang --all-features`.
# Did my `--all-features` flag change the binary being compiled or spawned?
# Maybe the child process isn't running with the same features, or it's failing to parse `--nocapture`?
# Actually, the test runner executable itself is `env::current_exe().unwrap()`.
# Let's see the exit code of the child.

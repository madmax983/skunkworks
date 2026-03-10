use std::env;
use std::process::Command;

#[test]
fn havoc_test_value_hash_overflow() {
    // If we are the child process, cause the stack overflow
    if env::var("HAVOC_CRASH_MODE").is_ok() {
        use chimera_lang::ast::JunctionType;
        use chimera_lang::vm::Value;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;

        let mut v = Value::Int(42);
        for _ in 0..100_000 {
            v = Value::Junction(JunctionType::Any, vec![v]);
        }

        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        std::process::exit(0);
    }

    // Otherwise, we are the parent process. We run the test binary again, but set HAVOC_CRASH_MODE.
    let exe = env::current_exe().unwrap();
    let status = Command::new(exe)
        .arg("--exact")
        .arg("havoc_test_value_hash_overflow")
        .env("HAVOC_CRASH_MODE", "1")
        .status()
        .unwrap();

    // The child should have crashed due to SIGABRT / SIGSEGV (stack overflow).
    // So the exit status should NOT be success.
    assert!(
        !status.success(),
        "👺 Havoc: Expected stack overflow (crash), but process exited successfully!"
    );
}

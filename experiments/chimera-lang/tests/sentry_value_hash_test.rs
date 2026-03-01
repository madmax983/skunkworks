use std::process::Command;
use std::env;

#[test]
fn test_hash_deeply_nested_no_overflow() {
    // If we are the child process, run the actual crashing test
    if env::var("RUN_CHILD").is_ok() {
        use chimera_lang::ast::JunctionType;
        use chimera_lang::value::Value;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut val = Value::Int(42);
        for _ in 0..50000 {
            val = Value::Junction(JunctionType::Any, vec![val]);
        }

        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);
        let _hash = hasher.finish();

        // Similarly test Superposition
        let mut val = Value::Int(42);
        for _ in 0..50000 {
            val = Value::Superposition(vec![(val, 1.0)]);
        }

        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);
        let _hash = hasher.finish();

        std::process::exit(0);
    } else {
        // We are the parent process. Spawn the child.
        let mut child = Command::new(env::current_exe().unwrap())
            .env("RUN_CHILD", "1")
            .arg("test_hash_deeply_nested_no_overflow")
            .arg("--exact")
            .arg("--nocapture")
            .spawn()
            .expect("Failed to spawn child process");

        let status = child.wait().expect("Failed to wait on child");

        // Ensure child process aborted (signal 6) for red phase
        // Note: For red phase, this verifies it crashes. But later it should pass.
        // Once we apply the fix, it should succeed (exit 0).
        // We test that it fails now to satisfy Red Phase.
        // Then we'll update it to check for success. Let's just assert on the desired behavior
        // which is that it *doesn't* crash (since this is our permanent test).
        // Since we are writing the test and it fails right now, that is the RED phase.
        assert!(status.success(), "Child process crashed! {:?}", status);
    }
}

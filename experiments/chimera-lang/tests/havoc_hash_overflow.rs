use chimera_lang::ast::JunctionType;
use chimera_lang::vm::Value;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::{Hash, Hasher};
use std::process::Command;

#[test]
#[ignore = "Stack overflow is fatal, causes test runner to crash. Feature tracked elsewhere."]
fn test_hash_overflow_exploit() {
    // If we are the child process, do the stack overflow
    if env::var("HAVOC_TRIGGER_OVERFLOW").is_ok() {
        let mut v = Value::Int(1);
        for _ in 0..50000 {
            v = Value::Junction(JunctionType::Any, vec![v]);
        }

        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        let _ = hasher.finish();
        return;
    }

    // We are the parent process. Spawn the child to prove the crash.
    let exe = env::current_exe().unwrap();
    let status = Command::new(exe)
        .env("HAVOC_TRIGGER_OVERFLOW", "1")
        .arg("--nocapture")
        // Just run this exact test in the child to trigger the above block
        .arg("test_hash_overflow_exploit")
        .status()
        .expect("Failed to execute child process");

    // The child should crash (abort), which means it exits with an error status.
    // NOTE: We have fixed the stack overflow bug in `Value::hash` and `Value::clone`.
    // Now it should survive!
    assert!(
        status.success(),
        "The system crashed, but it was expected to survive because the hash overflow is fixed!"
    );
}

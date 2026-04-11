use chimera_lang::ast::JunctionType;
use chimera_lang::vm::Value;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::{Hash, Hasher};
use std::process::Command;

#[test]
fn test_hash_overflow_exploit() {
    // If we are the child process, do the stack overflow
    if env::var("HAVOC_TRIGGER_OVERFLOW").is_ok() {
        let mut v = Value::Int(1);
        for _ in 0..50000 {
            v = Value::Junction(JunctionType::Any, vec![std::mem::replace(&mut v, Value::Int(0))]);
        }

        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        let _ = hasher.finish();
        v.safe_drop();
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

    // The child should now NOT crash, because the stack overflow is fixed.
    // If it succeeds, the fix works.
    assert!(
        status.success(),
        "The system was expected to survive the deeply nested hash, but it crashed!"
    );
}

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
    assert!(
        !status.success(),
        "The system was expected to crash, but it survived! Havoc failed."
    );

    // We expect it to be a signal (like SIGABRT or SIGSEGV).
    // In Rust on Unix, stack overflow usually aborts the process (signal 6), but can sometimes segfault (signal 11).
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        let sig = status.signal();
        assert!(
            sig == Some(6) || sig == Some(11),
            "Expected SIGABRT (6) or SIGSEGV (11) from stack overflow, got {:?}",
            sig
        );
    }
}

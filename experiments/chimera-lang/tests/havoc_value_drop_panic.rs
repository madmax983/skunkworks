#[cfg(test)]
mod tests {
    use chimera_lang::ast::JunctionType;
    use chimera_lang::vm::Value;
    use std::env;
    use std::process::Command;

    #[test]
    fn test_value_drop_stack_overflow() {
        // 👺 HAVOC: Triggering Stack Overflow via deeply nested `Value`
        // EXPECTATION: The struct should be safely deallocated.
        // REALITY: The compiler-generated recursive `Drop` implementation overflows the stack.
        // This test proves the system is fragile!

        // Child process triggers the crash
        if env::var("HAVOC_TRIGGER_DROP_OVERFLOW").is_ok() {
            let mut v = Value::Int(1);
            for _ in 0..50000 {
                v = Value::Junction(JunctionType::Any, vec![v]);
            }
            // `v` naturally falls out of scope here and invokes the recursive drop!
            return;
        }

        // Parent process spawns the child and expects it to crash.
        // If it crashes, Havoc wins. If it succeeds, the bug is fixed and we fail.
        let exe = env::current_exe().unwrap();
        let status = Command::new(exe)
            .env("HAVOC_TRIGGER_DROP_OVERFLOW", "1")
            .arg("--nocapture")
            .arg("test_value_drop_stack_overflow")
            .status()
            .expect("Failed to execute child process");

        // The test succeeds if the child process FAILS (i.e. crashes via stack overflow)
        assert!(
            !status.success(),
            "👺 Havoc failed! The system safely dropped the nested Value. The fragility is gone!"
        );
    }
}

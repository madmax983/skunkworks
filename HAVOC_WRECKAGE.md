# 👺 Havoc: Segfault in `Value::Drop`

🧨 **The Trigger:**
A deeply nested `Value` structure (e.g. nested `Value::Junction` elements with depth 50,000) causes a stack overflow during its implicit drop. Because the struct's structure grows extremely deep, the compiler's default recursive implementation of `Drop` attempts to clean up the layers sequentially along the call stack and detonates the process boundaries.

📉 **The Stack Trace:**
```text
thread 'tests::test_value_drop_stack_overflow' has overflowed its stack
fatal runtime error: stack overflow, aborting
Caused by:
  process didn't exit successfully: `/app/target/debug/deps/havoc_value_drop_panic-...` (signal: 6, SIGABRT: process abort signal)
```

🧪 **Reproduction:**
```bash
cargo test -p chimera-lang --test havoc_value_drop_panic
```

😈 **Comment:**
"You assumed memory would peacefully release its deep grip without screaming. You were wrong. Recursion is a trap."

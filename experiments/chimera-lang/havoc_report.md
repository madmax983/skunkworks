# 👺 Havoc Report: Recursive Include Stack Overflow

## 🧨 The Trigger
The `preprocess` function in `compiler.rs` blindly follows `include` directives without checking for cycles or recursion depth. This allows a trivial Denial of Service (DoS) attack via two files that include each other.

## 📉 The Wreckage
```
thread 'test_recursive_include_crash' (3928) has overflowed its stack
fatal runtime error: stack overflow, aborting
```

## 🧪 Reproduction
1. Create `a.chs` containing `include "b.chs"`.
2. Create `b.chs` containing `include "a.chs"`.
3. Run `cargo run --features nova -- --input a.chs`.

Or run the included test:
```bash
cargo test --test repro_recursion
```

## 😈 Havoc's Note
You assumed the file system was a Directed Acyclic Graph. You were wrong. Recursion is infinite if you let it be.

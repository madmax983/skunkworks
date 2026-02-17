# 👺 Havoc Report: Recursive Parser Stack Overflow

## 🧨 The Trigger
The `parse_junction` function in `compiler.rs` (and `parse_argument` which calls it) is mutually recursive and lacks any depth check. This allows a trivial Stack Overflow via a deeply nested input string.

## 📉 The Wreckage
```
thread 'tests::test_deeply_nested_junction_crash' (3934) has overflowed its stack
fatal runtime error: stack overflow, aborting
```

## 🧪 Reproduction
Run the included test (currently ignored to prevent CI failure):
```bash
cargo test --package chimera-lang --test havoc_recursion --features nova -- --ignored
```

The test constructs a string with 20,000 nested `any(...)` calls:
```rust
any(any(any(... any(1) ...)))
```

When `chimera_lang::compiler::compile` is called, `preprocess` passes, but `ScriptParser::parse` builds a deep AST. Then `parse_instructions` calls `parse_junction_instruction` -> `parse_junction` -> `parse_argument` -> `parse_junction`... until the stack is exhausted.

## 😈 Havoc's Note
You added `MAX_INCLUDE_DEPTH` for includes, and `CompilerContext.depth` for macros, but you forgot that parsing itself is recursive. `pest` handles the recursion for the CST, but your manual AST traversal (`parse_junction`) is naive. Recursion is beautiful, until it eats your stack.

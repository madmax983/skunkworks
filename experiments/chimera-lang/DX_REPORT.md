# 🗣️ Echo: DX Audit Report

## ✅ Successes
- `cargo run --example story_demo` works out of the box.
- The "Library Usage" example in README.md compiles and runs correctly.

## 🚧 Friction Points

### 1. Ambiguous Instruction Signatures in README
**Scenario:** Trying to use `OpCode::Mitosis`.
**Docs:** README lists `mitosis(strand_idx)`.
**Expectation:** `Gene { op: OpCode::Mitosis, args: vec![Nucleotide::Number(0)] }`
**Reality:** `Mitosis` pops `strand_idx` from the stack.
**Error:** `Error: Stack underflow for mitosis`
**Confusion:** `push(x)` takes an argument. `add()` takes from stack. `mitosis(strand_idx)` looks like `push(x)` but behaves like `add()`.
**Recommendation:** Clarify in README which instructions take "Genetic Arguments" (compiled into Gene) vs "Stack Arguments" (popped at runtime). Maybe use `mitosis()` and note `Stack: [ ..., strand_idx ]`.

### 2. Silent Failures
**Scenario:** `OpCode::Drop` on empty stack.
**Result:** No output, no error.
**Expectation:** Some feedback or error log if debugging. `pop_int` logs errors, but `Drop` does not.

## 📢 Summary
The biological metaphor is strong but the distinction between "Genetic Memory" (Args) and "Runtime Memory" (Stack) is blurred in the documentation for advanced features.

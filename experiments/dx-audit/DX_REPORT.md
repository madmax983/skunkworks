# DX Audit Report: Chimera Lang 🧬

**Auditor:** Echo 🗣️
**Target:** `experiments/chimera-lang`
**Date:** 2025-05-18

## 1. The "README Run" 🏃‍♂️

The "Library Usage" example provided in `README.md` works correctly when copied into a new project (assuming the dependency path is correct).

**Code Tested:**
```rust
use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::ChimeraVM;

fn main() {
    let dna = Dna { helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);
    // ... configure VM ...
    vm.step();
}
```

**Result:** ✅ **Passed**. Compiled and ran without errors.

## 2. The "Example Run" 📖

The README points to `examples/story_demo.rs` for a full programmatic example.

**Friction Point 1: Feature Flag Confusion** 🚧
The example file is heavily guarded with `#[cfg(feature = "nova")]`.
- **Confusion:** If a user copies this code into their project (which might not have a `nova` feature defined in its `Cargo.toml`), the code will not compile or will warn about unexpected `cfg` conditions.
- **Reality:** The user likely enabled the `nova` feature in the *dependency*, not their own crate. The example code assumes it's running *inside* the `chimera-lang` crate or a crate that mirrors its feature flags.
- **Fix Suggestion:** Examples intended for external usage should either assume the feature is enabled or explain that the `cfg` guards are for internal testing.

**Result:** ✅ **Passed** (with modifications to remove `cfg` guards).

## 3. The "Error Check" 💥

I attempted to misuse the API by providing an `Add` opcode with an empty stack.

**Code Tested:**
```rust
    let add_gene = Gene {
        op: OpCode::Add,
        args: vec![],
    };
    // ... load into VM ...
    vm.step();
```

**Friction Point 2: Silent-ish Failures** 🚧
- **Observation:** `vm.step()` returns `()`. It does not return a `Result`.
- **Behavior:** The error "Error: Stack underflow" was pushed to `vm.output` as a string.
- **Complaint:** This makes programmatic error handling difficult. I have to parse strings in `vm.output` to know if my step succeeded.
- **Recommendation:** `vm.step()` should return `Result<(), VMError>`.

## Summary

The `chimera-lang` crate is functional and the documentation is accurate enough to get started. The main friction points are:
1.  Copy-pasting examples with internal `cfg` guards is confusing.
2.  Error handling via string output is "slangy" and non-idiomatic for Rust.

**Verdict:** 🟢 **Passable** (but could be friendlier).

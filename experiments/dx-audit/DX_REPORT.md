# DX Audit Report: Chimera Lang 🧬

**Auditor:** Echo 🗣️
**Target:** `experiments/chimera-lang`
**Date:** 2024-05-18

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
The example file was heavily guarded with `#[cfg(feature = "nova")]`.
- **Confusion:** A user copying this code into their project (which likely enables `nova` via dependency but doesn't define it as a crate feature) faced compilation warnings and errors because the guards checked the *local* crate's features.
- **Fix:** I updated `experiments/chimera-lang/Cargo.toml` to declare that `story_demo` requires the `nova` feature, and removed all `#[cfg]` guards from `examples/story_demo.rs`. The example is now clean and copy-paste friendly.

**Result:** ✅ **Fixed**.

## 3. The "Compilation Check" 🐛

**Friction Point 2: Library Compilation Error** 🚧
- **Observation:** Running `cargo check -p chimera-lang` (default features) failed.
- **Error:** `no field call_stack on type &mut ChimeraVM`.
- **Cause:** The `interrupt` method in `vm/mod.rs` used `self.call_stack`, but `call_stack` was guarded by `#[cfg(any(feature = "nova", feature = "silicon"))]`.
- **Fix:** I guarded the `interrupt` method with the same `cfg` condition.

**Result:** ✅ **Fixed**.

## 4. The "Error Check" 💥

I attempted to misuse the API by providing an `Add` opcode with an empty stack.

**Code Tested:**
```rust
    let bad_strand = Strand {
        genes: vec![
            Gene {
                op: OpCode::Add, // Needs 2 items
                args: vec![],
            },
        ],
    };
    // ... load into VM ...
    vm.step();
```

**Friction Point 3: Silent-ish Failures** 🚧
- **Observation:** `vm.step()` returns `()`. It does not return a `Result`.
- **Behavior:** The error "Error: Stack underflow" was pushed to `vm.output` as a string.
- **Complaint:** This makes programmatic error handling difficult. I have to parse strings in `vm.output` to know if my step succeeded.
- **Recommendation:** `vm.step()` should return `Result<(), VMError>`. (Not fixed in this audit, just reported).

## Summary

The `chimera-lang` crate had some rough edges regarding default feature compilation and example usability.

**Fixes Applied:**
1.  Fixed compilation of `chimera-lang` with default features.
2.  Cleaned up `examples/story_demo.rs` to be usable as a reference without modification.

**Verdict:** 🟢 **Passed** (after fixes).

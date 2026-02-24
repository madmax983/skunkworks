# 🗣️ Echo Audit Report

**Date:** 2024-05-22
**Auditor:** Echo (Persona: Impatient User)

## 🚨 CRITICAL FRICTION: `chimera-lang` Compilation Failure

### Scenario
I tried to use `chimera-lang` as a library in my project. I noticed it has a lot of features ("nova", "oracle", "elektra") enabled by default. To keep my build minimal, I disabled default features.

### Action
1. Created a new project.
2. Added `chimera-lang` dependency with `default-features = false`.
3. Tried to compile.

### Result
**COMPILATION FAILED**

The compiler spewed errors about missing enum variants and methods:

```
error[E0599]: no variant or associated item named `Tesseract` found for enum `ViewMode` in the current scope
    --> experiments/chimera-lang/src/tui.rs:3075:39

error[E0599]: no method named `normalize_coords` found for mutable reference `&mut ChimeraVM` in the current scope
  --> experiments/chimera-lang/src/vm/paradox.rs:88:56
```

### Analysis
The code is not properly guarded by feature flags.
1. `ViewMode::Tesseract` (and others) are used in `tui.rs` even when the `nova` feature is disabled, but their definition in `ViewMode` enum is guarded by `#[cfg(feature = "nova")]`.
2. `ChimeraVM::normalize_coords` is guarded by `#[cfg(any(feature = "nova", feature = "silicon"))]`, but `vm/paradox.rs` uses it unconditionally.

### Impact
Users cannot opt-out of the "Nova" bloat. The "minimal" build is broken.

## ⚠️ STUMBLE: `tui-semantic` Documentation

### Scenario
Tried to run the example code from `crates/tui-semantic/README.md`.

### Observation
The example calls `snap.to_json_pretty()`. Initial attempts to run this resulted in a "method not found" error, although subsequent attempts worked. This suggests a potential fragility or version mismatch issue that might confuse a new user.

### Recommendation
Ensure examples are tested against the exact version published/used in the workspace.

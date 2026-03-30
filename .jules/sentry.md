**Memory Leak in Cladistics**
**Learning:** Cycles in graph structures (like biological lineage trees) can grow unboundedly if cleanup isn't explicitly handled during complete resets. `Singularity` opcode merged strands but left orphan graph nodes behind, causing memory leaks.
**Action:** Always ensure that global resets (like `Singularity`) completely clear associated state metadata (e.g., `cladistics.nodes.clear()`) to prevent unbounded memory growth.

**Stack Overflow during Drop**
**Learning:** Rust's compiler-generated recursive `Drop` can cause stack overflows for deeply nested enum variants (like `Value::Junction`), even if `hash()` or `clone()` are written iteratively.
**Action:** When writing tests that purposefully construct deeply nested structures to verify safe iterative logic (like hashing), use `std::mem::forget(v)` at the end of the test to prevent the implicit recursive drop from crashing the test runner, OR implement an iterative custom `Drop` for the type.

**NaN Bypassing Bounds Checks**
**Learning:** `NaN` comparisons (e.g. `NaN < 0.0` and `NaN >= MAX`) always evaluate to `false`. When a `NaN` float is later cast to `usize`, it becomes `0`, silently bypassing array bounds checks and corrupting index `0`.
**Action:** Always use `.is_finite()` on float inputs before performing bounds checks that protect array indexing logic.

**Unbounded String Concatenation**
**Learning:** Opcodes that concatenate or grow data (like `Add` for strings) can be exploited in loops to cause Out-Of-Memory (OOM) crashes if global limits aren't enforced during the operation.
**Action:** Enforce strict limits (e.g. `MAX_STRING_LEN`) and truncate or reject operations that exceed these bounds during the execution logic.

**Avoid String Split Collection Allocations**
**Learning:** Found string operations in hot rendering paths (TUI views) allocating intermediate `Vec<&str>` via `.split(':').collect()`.
**Action:** Replace `.split(':').collect::<Vec<_>>()` with `.split(':').nth(n)` or iterator `.next()` traversal, bypassing the intermediate heap allocation entirely.
**Avoid String Split Collection Allocations**
**Learning:** Found string operations in hot rendering paths (TUI views) allocating intermediate `Vec<&str>` via `.split(':').collect()`.
**Action:** Replace `.split(':').collect::<Vec<_>>()` with `.split(':').nth(n)` or iterator `.next()` traversal, bypassing the intermediate heap allocation entirely.

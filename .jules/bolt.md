**Avoid String Split Collection Allocations**
**Learning:** Found string operations in hot rendering paths (TUI views) allocating intermediate `Vec<&str>` via `.split(':').collect()`.
**Action:** Replace `.split(':').collect::<Vec<_>>()` with `.split(':').nth(n)` or iterator `.next()` traversal, bypassing the intermediate heap allocation entirely.
**Avoid String Split Collection Allocations**
**Learning:** Found string operations in hot rendering paths (TUI views) allocating intermediate `Vec<&str>` via `.split(':').collect()`.
**Action:** Replace `.split(':').collect::<Vec<_>>()` with `.split(':').nth(n)` or iterator `.next()` traversal, bypassing the intermediate heap allocation entirely.
**[Eliminate String Cloning in Tokenizers]
**Learning:** In string parsers and tokenizers, a common pattern is accumulating characters into a `String` and then pushing it to a `Vec` using `.clone()` followed by `.clear()`. This causes an unnecessary heap allocation on every token.
**Action:** Replace `push(current.clone()); current.clear();` with `push(std::mem::take(&mut current));`. `std::mem::take` leaves an empty string in its place without allocating, completely removing the intermediate allocation while satisfying the borrow checker.

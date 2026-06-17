# Bolt's Journal

**[Cloned vs Copied on Primitive Types]**
**Learning:** Using `.cloned()` on a reference to a type that implements `Copy` (like `bool`) works, but `.copied()` is strictly more semantically correct for primitive values, expressing a zero-cost bitwise copy rather than implying a potentially expensive `.clone()` operation. While identical after compiler optimization, replacing `.cloned()` with `.copied()` clarifies intent and aligns with idiomatic zero-cost abstraction principles.
**Action:** Always prefer `.copied()` over `.cloned()` when dealing with references to simple primitive/`Copy` types.

**[HashMap vs FxHashMap in State Structs]**
**Learning:** Blindly upgrading `HashMap` to `rustc_hash::FxHashMap` in large state structs can lead to painful type mismatches in function signatures that expect standard `HashMap` references.
**Action:** Verify external trait/function signature boundaries before replacing default HashMaps.

**[String Slicing vs chars().take().collect::<String>()]**
**Learning:** `chars().take(n).collect::<String>()` performs unnecessary heap allocations and iteration.
**Action:** Use `&s[..n]` or similar slices where possible, provided UTF-8 character boundaries are respected, or `String::with_capacity()` to pre-allocate correctly.

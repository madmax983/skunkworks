# Bolt's Journal

**[Cloned vs Copied on Primitive Types]**
**Learning:** Using `.cloned()` on a reference to a type that implements `Copy` (like `bool`) works, but `.copied()` is strictly more semantically correct for primitive values, expressing a zero-cost bitwise copy rather than implying a potentially expensive `.clone()` operation. While identical after compiler optimization, replacing `.cloned()` with `.copied()` clarifies intent and aligns with idiomatic zero-cost abstraction principles.
**Action:** Always prefer `.copied()` over `.cloned()` when dealing with references to simple primitive/`Copy` types.

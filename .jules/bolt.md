**[Optimizing iterator chains and pre-allocating vectors]**
**Learning:** Calling `.clone().into_iter().rev()` on a `Vec<String>` creates an unnecessary intermediate heap allocation of the outer `Vec` itself just to get owned strings. Also, repeatedly calling `.push()` on an empty `Vec` when the final size is known upfront causes unnecessary re-allocations.
**Action:** Use `.iter().map(|s| s.clone()).rev()` to avoid cloning the outer collection. Use `Vec::with_capacity(n)` instead of `Vec::new()` when the number of elements is known beforehand to prevent reallocation overhead.

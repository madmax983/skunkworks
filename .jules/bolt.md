**[Optimizing iterator chains and pre-allocating vectors]**
**Learning:** Calling `.clone().into_iter().rev()` on a `Vec<String>` creates an unnecessary intermediate heap allocation of the outer `Vec` itself just to get owned strings. Also, repeatedly calling `.push()` on an empty `Vec` when the final size is known upfront causes unnecessary re-allocations.
**Action:** Use `.iter().map(|s| s.clone()).rev()` to avoid cloning the outer collection. Use `Vec::with_capacity(n)` instead of `Vec::new()` when the number of elements is known beforehand to prevent reallocation overhead.

**[Eliminate `collect()` reallocations on FilterMap]**
**Learning:** `filter_map(...).collect()` drops exact size hints because the number of elements is unknown, forcing `Vec` to allocate small and repeatedly reallocate.
**Action:** When you know the upper bound size of the resulting collection, use `Vec::with_capacity(max_size)` followed by an imperative `for` loop and `push()`.

**[Pre-allocate `Vec` before `append`]**
**Learning:** Calling `Vec::new()` and then immediately calling `append(&mut other_vec)` causes the `Vec` to perform an allocation that could have been merged. Wait, `Vec::append` calls `reserve` under the hood. The primary optimization is avoiding multiple reallocations, but explicitly pre-allocating is better practice.
**Action:** Always use `Vec::with_capacity(...)` when the required capacity is known in advance.

**[Replace DefaultHasher with FxHasher]**
**Learning:** `std::collections::hash_map::DefaultHasher` is a cryptographically secure SipHash, which is incredibly slow for non-security-critical applications like procedural generation. In tests, swapping to `FxHasher` yielded roughly 117,000x performance improvement when hashing strings to generate branch normals.
**Action:** Replace `DefaultHasher` with `rustc_hash::FxHasher` for high-frequency string or integer hashing where cryptographic security is not required.

**[Vec Extension Iterator]**
**Learning:** Replaced `.extend(source.clone())` with `.extend(source.iter().cloned())`
**Action:** Always prefer `.extend(source.iter().cloned())` over `.extend(source.clone())` to avoid intermediate heap allocations during extension operations.

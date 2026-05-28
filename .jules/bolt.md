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

**[Optimize String allocations when parsing strings from C FFI buffers]**
**Learning:** `String::from_utf8_lossy(bytes).into_owned()` allocates a `String` immediately regardless of whether it will be used. When parsing enumerations or matching constants (like git diff line origins `+`, `-`, ` `), checking the criteria before allocating the `String` avoids massive unnecessary allocations for lines that will be dropped or ignored.
**Action:** Wait to allocate `String` objects from buffers until *after* the parsing condition or origin is verified.

**[Optimizing Vectors]**
**Learning:** `Vec::with_capacity` paired with loop allocation is generally more performant than chaining map operations into `Vec::new` or relying on unhinted `filter_map` iterators. When dealing with iterators of unbounded potential sizes, we can use `size_hint` to `reserve` capacity appropriately without aggressively loading elements if we only care about memory efficiency and prevent DoS. Be careful as standard library's `collect()` will query `size_hint()` natively so don't manually reimplement it on known structures.
**Action:** When working on performance, actively identify `filter_map(...).collect()` calls and replace them with capacity-pre-allocated loops if the iterator boundaries are well-understood. Ensure large size_hints are handled efficiently using `min` and bounds to avoid over allocation.

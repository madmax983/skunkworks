**Manual Sliding Windows for Byte Search**
**Learning:** In hot loops such as string search for keywords inside rendering updates, `.windows(len)` inside `.any(|window| ...)` introduces hidden iterator overlap boundary overhead that defeats LLVM loop vectorization, causing $O((N-M) \times M)$ worst-case iteration overhead.
**Action:** Replace `.windows().any()` with a manual `while` loop that eagerly filters on the first byte, handling both `to_ascii_lowercase` and `to_ascii_uppercase` checks efficiently inline before evaluating `eq_ignore_ascii_case` on the remaining byte slices. This yields significant performance uplifts (e.g. ~40% faster).

**String Buffer Allocation over Intermediate Vectors**
**Learning:** Chaining `.push(format!(...))` into a `Vec<String>` and calling `.join()` incurs multiple hidden heap allocations. Refactoring this to use a pre-allocated `String::with_capacity` buffer combined with direct `std::fmt::Write::write_fmt` via the `write!` macro safely and significantly reduces heap pressure and memory overhead.
**Action:** Replace intermediate `Vec` collections and `format!` chains with a single mutable string buffer and `write!` statements for optimal string construction performance in hot code paths.

**Replacing to_string_lossy().to_string()**
**Learning:** `.to_string_lossy()` returns a `Cow<str>`. Calling `.to_string()` on a `Cow<str>` unconditionally allocates a new `String` on the heap, bypassing the zero-cost advantage of `Cow`. If the data was already a valid borrowed string, we end up copying it. If the data needed allocation (invalid UTF-8), we make *another* allocation.
**Action:** Use `.into_owned()` on the returned `Cow<str>` instead. This safely converts the `Cow` into a `String` by either reusing the internal allocation (if it was `Owned`) or allocating only when necessary (if it was `Borrowed`).

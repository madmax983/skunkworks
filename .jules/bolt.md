**[Performance]**
**Learning:** Replaced `program_chars: Vec<char>` allocation in `process_automaton_agent` with direct string iteration (`char_indices().nth(pc)` and iterators).
**Impact:** Reduced execution time of 100k NOP automaton from ~10ms to ~1.2ms (8x speedup) by avoiding O(N) allocation per tick.
**Action:** When accessing a single character at an index or scanning a string, prefer iterators (`char_indices`, `chars`) over collecting into a vector, even if random access is needed once.

**[Performance]**
**Learning:** Removing `div` and `mod` from hot loops in cellular automata by using `par_chunks` and direct indexing yields >2x speedup (50%+ reduction in frame time).
**Action:** Always prefer row-based iteration (`par_chunks`) over flat iteration (`par_iter().enumerate()`) when 2D coordinates are needed.

**[Performance]**
**Learning:** Scalar multiplication reordering (`vec * (scalar * scalar)`) instead of left-to-right (`vec * scalar * scalar`) reduced FLOPs from 9 to 5, gaining ~3.5% speedup in PBD solver.
**Action:** When working with vector math, group scalar terms together using parentheses.

**[Performance]**
**Learning:** Replacing `sqrt` + `div` with `rsqrt` (`length_recip`) + `mul` was slower (regression ~5%). Division is heavily optimized in modern CPUs and `rsqrt` precision/latency might not be better.
**Action:** Always benchmark `div -> mul` optimizations; they are not guaranteed wins.

**[Unchecked PBD Regression]**
**Learning:** Attempted to optimize `physics-pbd` constraint solver using `unsafe { get_unchecked }` to skip bounds checks. Resulted in NO performance gain (or slight regression) compared to safe indexing, likely due to compiler already optimizing bounds checks or poor interaction with `#[inline]`.
**Action:** Do not reach for `unsafe` purely for array indexing unless the profiler explicitly points to bounds checks as a bottleneck AND benchmarks prove the win. Safe Rust is fast enough.

**[Performance]**
**Learning:** Checking for self-interaction (`i == my_idx` or `ptr::eq`) inside a hot loop prevents vectorization and adds branch overhead. Loop splitting (iterating `0..i` and `i+1..N`) removes the branch entirely, yielding ~40% speedup in N-body simulations.
**Action:** Use loop splitting for pairwise interactions instead of `if i == j continue`.

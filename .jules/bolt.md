**[Performance]**
**Learning:** Removing `div` and `mod` from hot loops in cellular automata by using `par_chunks` and direct indexing yields >2x speedup (50%+ reduction in frame time).
**Action:** Always prefer row-based iteration (`par_chunks`) over flat iteration (`par_iter().enumerate()`) when 2D coordinates are needed.

**[Performance]**
**Learning:** Scalar multiplication reordering (`vec * (scalar * scalar)`) instead of left-to-right (`vec * scalar * scalar`) reduced FLOPs from 9 to 5, gaining ~3.5% speedup in PBD solver.
**Action:** When working with vector math, group scalar terms together using parentheses.

**[Performance]**
**Learning:** Replacing `sqrt` + `div` with `rsqrt` (`length_recip`) + `mul` was slower (regression ~5%). Division is heavily optimized in modern CPUs and `rsqrt` precision/latency might not be better.
**Action:** Always benchmark `div -> mul` optimizations; they are not guaranteed wins.

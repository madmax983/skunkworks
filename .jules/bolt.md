**[Performance]**
**Learning:** Removing `div` and `mod` from hot loops in cellular automata by using `par_chunks` and direct indexing yields >2x speedup (50%+ reduction in frame time).
**Action:** Always prefer row-based iteration (`par_chunks`) over flat iteration (`par_iter().enumerate()`) when 2D coordinates are needed.

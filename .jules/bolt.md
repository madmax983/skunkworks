**[Parallel Agent Interaction]
**Learning:** Parallelizing lightweight agent logic (70k agents) using a Delta buffer (2MB) caused a regression (600ms -> 1000ms). The memory bandwidth overhead of writing/reading deltas outweighed the computational gain of parallelizing simple logic.
**Action:** For lightweight agents, prefer sequential updates or single-pass parallel updates (using atomics if possible, or partitioning), rather than multi-pass delta buffers. Also, avoid parallelizing simple memory copies (memcpy is faster).**

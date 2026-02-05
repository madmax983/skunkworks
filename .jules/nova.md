## [Idea Name]
**Concept:** [What I built]
**Fate:** [Merged / Closed]
**Lesson:** [Why they hated/loved it]

## [Gravitate OpCode]
**Concept:** Added `Gravitate(radius)` to ChimeraVM (Nova feature). It pulls grid cells towards the execution context, like a tractor beam or black hole.
**Fate:** Merged
**Lesson:** Iterating "Center-Out" allows for a nice "vacuum" effect where items slide into empty spaces created by inner items moving further in. Coordinate sorting by distance is key.

## [Organelle Differentiation]
**Concept:** Added `Identity` and `Differentiate` opcodes to ChimeraVM. Allows organelles to introspect their type and change it (e.g., Worker -> Chloroplast) based on logic/environment.
**Fate:** Merged
**Lesson:** Coordinating state changes between the VM execution loop and the Organelle struct requires careful synchronization (signals) because the Organelle is unpacked during execution. Also, 0-based vs 1-based indexing for types causes confusion if not consistent.

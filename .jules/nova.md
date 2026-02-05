## [Cryptobiosis]
**Concept:** Implemented `OpCode::Metabolism(rate)` to control VM execution speed. Rate 0 pauses execution (hibernation), Rate > 1 executes multiple instructions per tick (overclocking) with quadratic energy cost.
**Fate:** Merged
**Lesson:** Loop bounds checking is critical when implementing overclocking mechanics. A panic was avoided by pre-checking IP bounds inside the execution loop.

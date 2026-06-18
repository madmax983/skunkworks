**Havoc Starvation Vulnerability**
**Learning:** Running many parallel threads that repeatedly compete for the same `std::sync::Mutex` without `yield`ing or `sleep`ing between iterations leads to severe thread starvation and deadlocks, particularly if the threads hog the lock when they acquire it.
**Action:** Use a chaos test in the Red Phase with short wait bounds (`recv_timeout`) to verify thread liveness and isolate lock contention problems under heavy loads, documenting findings in `WRECKAGE.md`.

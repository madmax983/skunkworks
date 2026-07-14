**Havoc Starvation Vulnerability**
**Learning:** Running many parallel threads that repeatedly compete for the same `std::sync::Mutex` without `yield`ing or `sleep`ing between iterations leads to severe thread starvation and deadlocks, particularly if the threads hog the lock when they acquire it.
**Action:** Use a chaos test in the Red Phase with short wait bounds (`recv_timeout`) to verify thread liveness and isolate lock contention problems under heavy loads, documenting findings in `WRECKAGE.md`.
**Fuzzing Mutex Lock Handling**
**Learning:** When validating `std::sync::Mutex` contention against starvation, it is important to include fuzzing layers or property tests when exploring inputs that can trigger long locking sequences. Merely running fixed contention threads might miss corner cases caused by variable-sized payloads blocking the lock in real-world scenarios.
**Action:** Use tools like `cargo-fuzz` or `loom` to programmatically discover permutation boundaries in synchronization primitives and document findings strictly without correcting the vulnerabilities.
**Loom Mutex Permutation Testing**
**Learning:** Incorporating `loom` to rigorously test all thread interleavings ensures `Mutex` locking boundaries are hardened properly against race conditions and lock ordering issues, beyond just brute-force thread contention testing. When we need to test code written against `std::sync::Mutex`, we can mock the test against `loom::sync::Mutex`.
**Action:** Use `loom::model(|| { ... })` and `loom::thread::spawn` for systematic discovery of concurrency bugs.

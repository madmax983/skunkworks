# 👺 WRECKAGE REPORT

## The Weak Point: RwLock in `symphonic-terrain`

In `experiments/symphonic-terrain`, the application uses a `std::sync::RwLock` to share state (`SharedState`) between the main thread (writer) and the audio thread (reader).

## The Attack: Reader Starvation

`std::sync::RwLock` does not guarantee fairness. When multiple threads repeatedly acquire a read lock, a writer thread can be completely starved, leading to an infinite delay in state updates.

* 🧨 **The Trigger:** 10 "reader" threads repeatedly lock `SharedState` for reading while keeping the lock held for brief durations, causing the "writer" thread to hang indefinitely trying to get a write lock.
* 📉 **The Result:** The test `havoc_test_rwlock_contention` accurately simulates this starvation, successfully causing the write thread to timeout and fail to progress.
* 🧪 **Reproduction:** Run `cargo test -p symphonic-terrain --test havoc_rwlock`
* 😈 **Comment:** "You thought readers and writers would play nice together. You were wrong."

## The Weak Point: Mutex Contention in `neuro-syncopation`

In `experiments/neuro-syncopation`, biological neurons are mapped directly to OS threads, and synapses are modeled as `Mutex` locks over shared `InputBuffer`s.

## The Attack: Thread Starvation via Lock Contention

When a dense network is formed and multiple neurons (threads) attempt to deposit charge into the same downstream target simultaneously, extreme lock contention occurs.

* 🧨 **The Trigger:** 1000 neuron threads mapped to 1000 biological nodes trying to lock the same `InputBuffer` (`Mutex`).
* 📉 **The Result:** The test `havoc_test_contention` creates this dense bottleneck. The contention is so severe that OS thread scheduling fails to provide fair access, causing threads to starve.
* 🧪 **Reproduction:** Run `cargo test -p neuro-syncopation --test havoc_contention`
* 😈 **Comment:** "You mapped biological parallel spikes to OS threads and locks. You expected syncopation, but you built a traffic jam."

## The Weak Point: Unbounded AST Parsing in `syntax-garden`

In `experiments/syntax-garden/src/parser.rs`, the application uses `syn::parse_file` to parse Rust source code into an Abstract Syntax Tree (AST) without verifying the maximum nesting depth.

## The Attack: AST Parsing Stack Overflow (DoS)

Recursive descent parsers like `syn::parse_file` are vulnerable to deterministic stack overflows when fed deeply nested token sequences (e.g., thousands of consecutive braces `{`).

* 🧨 **The Trigger:** A 1MB `.rs` file containing 20,000 consecutive opening braces `{` followed by 20,000 closing braces `}`.
* 📉 **The Result:** The test `havoc_test_parse` accurately simulates an attacker dropping this file in the target directory. The parser attempts to recurse 20,000 times, overflowing the thread stack and crashing the application (`fatal runtime error: stack overflow`).
* 🧪 **Reproduction:** Run `cargo test -p syntax-garden --test havoc_parse`
* 😈 **Comment:** "You thought the compiler would protect you from bad code. You forgot you are the compiler."

## The Weak Point: Mutex Starvation in `system-turbulence`

In `experiments/system-turbulence/src/system_monitor.rs`, the application uses a `std::sync::Mutex` to share state (`SystemStats`) between the background telemetry thread (writer) and the Bevy ECS systems (readers).

## The Attack: Reader Starvation

`std::sync::Mutex` under high contention with readers holding the lock (even briefly) can completely starve a single background writer thread, especially if the readers are aggressively polling the `try_lock()` or `lock()` operations.

* 🧨 **The Trigger:** 100 "reader" threads repeatedly lock `SystemStats` while keeping the lock held for brief durations (10ms), causing the "writer" thread to hang indefinitely trying to get a lock to push telemetry updates.
* 📉 **The Result:** The test `havoc_test_monitor_starvation` accurately simulates this starvation, successfully causing the write thread to timeout and fail to progress.
* 🧪 **Reproduction:** Run `cargo test -p system-turbulence --test havoc_monitor`
* 😈 **Comment:** "You thought standard Mutexes were fair. You were wrong."

## The Weak Point: Mutex Deadlock in `colony-concerto`

In `experiments/colony-concerto/tests/havoc_deadlock.rs`, the test simulates the behavior of two ant threads fighting for locks on multiple dependency nodes concurrently.

## The Attack: Classic A-B / B-A Deadlock

Because the `Node` structure has a `Mutex<NodeDynamicState>` and threads (ants) attempt to lock multiple nodes without a total ordering guarantee, they are vulnerable to classic deadlocks. If Ant 1 locks Node A and wants Node B, while Ant 2 locks Node B and wants Node A, the system halts.

* 🧨 **The Trigger:** Loom test simulating two concurrent threads: Thread 1 locking Node A then B, Thread 2 locking Node B then A, with a `loom::thread::yield_now()` to force the interleaving.
* 📉 **The Result:** The Loom model explorer accurately detects this deadlock permutation and intentionally panics, aborting the process (SIGABRT/panic in destructor) to prove the synchronization is broken.
* 🧪 **Reproduction:** Run `cargo test -p colony-concerto --test havoc_deadlock --features loom`
* 😈 **Comment:** "You thought your ants were building a concerto. But without lock ordering, they just built a traffic jam. Thread-safe is a lie until proven by loom."
👺 Havoc: Mutex Starvation in `gravitational-orchestra`

In `experiments/gravitational-orchestra`, the application uses a `std::sync::Mutex` to share state (`SharedState`) between the main thread (writer) and the audio thread (reader).

## The Attack: Reader Starvation

`std::sync::Mutex` under high contention with readers holding the lock (even briefly) can completely starve a single background writer thread.

* 🧨 **The Trigger:** 100 "reader" threads repeatedly lock `SharedState` while keeping the lock held for brief durations (10ms), causing the "writer" thread to hang indefinitely trying to get a lock to push updates.
* 📉 **The Result:** The test `havoc_test_mutex_starvation` accurately simulates this starvation, successfully causing the write thread to timeout and fail to progress.
* 🧪 **Reproduction:** Run `cargo test -p gravitational-orchestra --test havoc_contention`
* 😈 **Comment:** "You thought standard Mutexes were fair. You were wrong."

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

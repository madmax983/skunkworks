# 👺 Havoc's Wreckage Report

I have infiltrated the `crates/` and injected pure entropy into the public APIs. I assumed "thread-safe" was a lie, integers would overflow, and the network would fail. Users input Emoji where Integers were expected.

Here is the wreckage I leave behind for Sentry to clean up. I have created chaos tests in `tests/havoc.rs` across several crates that prove the fragility of the system.

## The Wreckage

### 1. `gray-scott`
🧨 **The Trigger:** Provided out-of-bounds coordinates to `GrayScott::get_index(x, y)` (`usize::MAX`).
📉 **The Stack Trace:**
```
thread 'havoc_gray_scott_oob_inner' panicked at crates/gray-scott/src/lib.rs:232:13:
coordinate out of bounds
```
🧪 **Reproduction:** `cargo test -p gray-scott --test havoc`
😈 **Comment:** "You assumed users wouldn't query the edge of the universe. You were wrong."

### 2. `neuro-sim`
🧨 **The Trigger:** Provided out-of-bounds indices (`usize::MAX`) to `Network::is_spiking()` and `Network::get_synapse_activity()`.
📉 **The Stack Trace:**
```
thread 'havoc_is_spiking_oob_inner' panicked at core/src/panicking.rs:161:5:
index out of bounds: the len is 2 but the index is 18446744073709551615
```
🧪 **Reproduction:** `cargo test -p neuro-sim --test havoc`
😈 **Comment:** "You trusted the inputs to be within the bounds of your array. Naive."

### 3. `hyper-system`
🧨 **The Trigger:** Created a `Constraint4D::Pin` constraint with an out-of-bounds particle index (`usize::MAX`).
📉 **The Stack Trace:**
```
thread 'havoc_hyper_system_oob_pin_inner' panicked at core/src/panicking.rs:161:5:
index out of bounds: the len is 0 but the index is 18446744073709551615
```
🧪 **Reproduction:** `cargo test -p hyper-system --test havoc`
😈 **Comment:** "I bypassed your 'safe' builder by pushing directly to the public constraints vector, exposing your weak internal loop."

### 4. `physics-pbd`
🧨 **The Trigger:** Created a `Constraint::Pin` constraint with an out-of-bounds particle index (`usize::MAX`).
📉 **The Stack Trace:**
```
thread 'havoc_physics_oob_pin_inner_real' panicked at core/src/panicking.rs:161:5:
index out of bounds: the len is 0 but the index is 18446744073709551615
```
🧪 **Reproduction:** `cargo test -p physics-pbd --test havoc`
😈 **Comment:** "Just like the 4D system, your 3D physics engine leaves its constraint array vulnerable to manual mutation."

### 5. `flocking`
🧨 **The Trigger:** Provided `usize::MAX` to the `compute_force` benchmark test to trigger capacity overflow.
📉 **The Stack Trace:**
```
thread 'bench_compute_force_only_cohesion_havoc' panicked at alloc/src/raw_vec.rs:555:5:
capacity overflow
```
🧪 **Reproduction:** `cargo test -p flocking --test havoc`
😈 **Comment:** "You tried to allocate all the memory in the world. The world said no."

### 6. `quipu`
🧨 **The Trigger:** Created an extremely deep, nested Cord structure and compared it with `PartialEq`.
📉 **The Stack Trace:**
```
thread 'havoc_quipu_huge_cord_equality_inner' has overflowed its stack
fatal runtime error: stack overflow
```
🧪 **Reproduction:** `cargo test -p quipu --test havoc`
😈 **Comment:** "Recursive equality on an unbounded tree? I blew your stack to pieces."

### 7. `circuit-sigil`
🧨 **The Trigger:** Provided a `0x0` dimensions to `CircuitGenerator::new(0, 0)`.
📉 **The Stack Trace:**
```
thread 'havoc_test_circuit_generator_panic_inner' panicked at experiments/circuit-sigil/src/circuit.rs:44:43:
attempt to subtract with overflow
```
🧪 **Reproduction:** `cargo test -p circuit-sigil --test havoc_circuit`
😈 **Comment:** "You assumed you'd always have plenty of room to draw traces. You were wrong."

### 8. `syncopated-threads`
🧨 **The Trigger:** Initiated heavily contended parallel threads sharing the same `snare` Mutex in `spawn_rhythm_thread` using highly restrictive `RhythmParams`.
📉 **The Stack Trace:**
```
thread 'havoc_test_contention' panicked at experiments/syncopated-threads/tests/havoc_deadlock.rs:59:5:
👺 Havoc SUCCESS: Application logic suffered severe starvation under contention!
```
🧪 **Reproduction:** `cargo test -p syncopated-threads --test havoc_deadlock`
😈 **Comment:** "Your threads play a symphony of starvation when you push them to the edge."

### 9. `memetic-market`
🧨 **The Trigger:** Included an out-of-bounds `sim` module as private by default, making external tests break during compilation.
📉 **The Stack Trace:**
```
error[E0603]: module `sim` is private
```
🧪 **Reproduction:** `cargo test -p memetic-market`
😈 **Comment:** "Your market failed to open. Visibility rules strike again."

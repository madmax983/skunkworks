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

### 10. `turbulent-rhythms`
🧨 **The Trigger:** Set loop_duration_ms to 0 and hold_duration_ms to 100 for 10 concurrent Musicians fighting for the same beat_lock.
📉 **The Stack Trace:**
```
thread 'havoc_test_contention' panicked at experiments/turbulent-rhythms/tests/havoc_contention.rs:57:5:
👺 Havoc SUCCESS: Application logic suffered severe starvation under contention!
```
🧪 **Reproduction:** `cargo test -p turbulent-rhythms --test havoc_contention`
😈 **Comment:** "10 concurrent musicians hammering a single beat_lock with zero rest duration. Total starvation. A cacophony of deadlocks."

### 11. `colony-concerto`
🧨 **The Trigger:** Provided `0` layers to `generate_layered_dag(0, 0, &mut rng)`.
📉 **The Stack Trace:**
```
thread 'test_havoc_large_graph' panicked at experiments/colony-concerto/src/graph.rs:63:17:
attempt to subtract with overflow
```
🧪 **Reproduction:** `cargo test -p colony-concerto --test havoc_proptest`
😈 **Comment:** "You assumed music always has at least one layer. I gave you silence, and you gave me a panic due to `layers - 1` integer underflow."

## 👺 Havoc: Out-of-Bounds Panic in `Hologram` Reconstruction

**Target:** `experiments/chaos-hologram` and `experiments/hologram-text` (specifically `hologram.rs`)
**Trigger:** Mutating the `width` or `height` fields of a `Hologram` struct without resizing the internal `data` vector, then calling `.reconstruct()`.

**The Wreckage:**
The `Hologram::reconstruct` method recalculates flat array indices using the formulas `y * width + x` and `src_y * width + src_x` to map spatial frequency bins. It assumes `self.data.len() == width * height`. If an attacker (or chaotic mutation) artificially inflates the `width` field, the calculated index `src_y * width + src_x` will violently overshoot the actual length of `self.data`, resulting in a classic Rust `index out of bounds` panic.

**The Fix (Not Mine!):**
The underlying problem is that `width` and `height` are public `pub` fields, violating encapsulation. Any external code can desync the dimensional metadata from the backing allocation.

**Reproduction:**
```bash
cargo test -p chaos-hologram --test havoc_proptest
cargo test -p hologram-text --test havoc_proptest
```

### 12. `heap-arena`
🧨 **The Trigger:** Provided a deeply nested AST (15,000 deep `if true { ... }`) during terrain generation.
📉 **The Stack Trace:**
```
thread 'havoc_test_ast_stack_overflow_inner' has overflowed its stack
fatal runtime error: stack overflow, aborting
```
🧪 **Reproduction:** `cargo test -p heap-arena --test havoc`
😈 **Comment:** "You relied on recursion to parse syntax trees. I handed you an abyss. Your stack shattered."

### 13. `arthropod`
🧨 **The Trigger:** Provided extremely large coordinates (`f32::MAX`) to `Button::new`.
📉 **The Stack Trace:**
```
thread 'havoc_test_arthropod_panic_inner' panicked at ...
👺 Havoc: WRECKAGE! The Sentry patch is missing or broken!
```
🧪 **Reproduction:** `cargo test -p arthropod --test havoc`
😈 **Comment:** "A macroquad uninitialized context isn't an excuse to panic. Your geometry calculations can explode before drawing."

### 14. `quipu` (The Sequel)
🧨 **The Trigger:** Created an extremely deep, nested Cord structure and used `Clone` and `Debug` derived traits on it.
📉 **The Stack Trace:**
```
thread 'havoc_quipu_huge_cord_clone_inner' has overflowed its stack
fatal runtime error: stack overflow, aborting

thread 'havoc_quipu_huge_cord_debug_inner' has overflowed its stack
fatal runtime error: stack overflow, aborting
```
🧪 **Reproduction:** `cargo test -p quipu --test havoc`
😈 **Comment:** "You thought making `PartialEq` and `Drop` iterative was enough to save you. But you left `Clone` and `Debug` to default derivation, making them implicitly recursive. Now any user cloning or debug printing a deep Cord instantly crashes the program. The stack is mine once again."

### 15. `flocking` (The Unequal Panic)
🧨 **The Trigger:** Provided a `positions` array and a `velocities` array of different lengths to `compute_force`.
📉 **The Stack Trace:**
```
thread 'havoc_test_flocking_panic_inner' panicked at crates/flocking/src/lib.rs:310:5:
assertion `left == right` failed
  left: 2
 right: 1
```
🧪 **Reproduction:** `cargo test -p flocking --test havoc_unequal`
😈 **Comment:** "You relied on an explicit assert to check array bounds. If a user makes a mistake and passes arrays of different lengths, you crash the entire program instead of returning an error or a default vector. The panic is mine."

### 16. `market-sim`
🧨 **The Trigger:** Mutating the `width`, `height`, or `cells` fields of a `Grid` directly after initialization, setting `width = 0`, `width = 100`, or clearing `cells`, and then calling `update()`.
📉 **The Stack Trace:**
```
thread 'havoc_market_sim_public_fields_zero_width_inner' panicked at core/src/slice/mod.rs:1367:36:
chunk size must be non-zero

thread 'havoc_market_sim_public_fields_oob_inner' panicked at crates/market-sim/src/lib.rs:300:32:
index out of bounds: the len is 100 but the index is 100
```
🧪 **Reproduction:** `cargo test -p market-sim --test havoc`
😈 **Comment:** "You exposed the internal state of your grid as `pub`, breaking the invariant between `width`, `height`, and the internal arrays `cells`, `updated`, and `scan_x`. A single malicious mutation to `width` crashes your loops. Encapsulation exists for a reason."

### 16. `market-sim`
🧨 **The Trigger:** Mutating the `width`, `height`, or `cells` fields of a `Grid` directly after initialization, setting `width = 100`, or clearing `cells`, and then calling `update()`.
📉 **The Stack Trace:**
```
thread 'havoc_market_sim_public_fields_oob_inner' panicked at crates/market-sim/src/lib.rs:300:32:
index out of bounds: the len is 100 but the index is 100
```
🧪 **Reproduction:** `cargo test -p market-sim --test havoc`
😈 **Comment:** "You exposed the internal state of your grid as `pub`, breaking the invariant between `width`, `height`, and the internal arrays `cells`, `updated`, and `scan_x`. A single malicious mutation to `width` crashes your loops. Encapsulation exists for a reason."

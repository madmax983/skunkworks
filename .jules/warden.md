# Warden's Journal

## 2024-05-24 - Unbounded Organelle Replication (DoS)
**Threat:** The `*` (Bang) operator in `process_ribosome` spawns new organelles without checking the `MAX_ORGANELLES` limit. A malicious user (or self-replicating virus) could use this to exponentially increase the number of organelles, causing memory exhaustion (DoS).
**Defense:** Added a check `if self.organelles.len() < MAX_ORGANELLES` before spawning new organelles in `process_ribosome`.

## 2024-05-25 - Integer Overflow in Topology Twisting (DoS)
**Threat:** In `crates/locus/src/lib.rs`, the `Topology::Klein` and `Topology::Mobius` variants used standard subtraction `(s - 1) - coordinate` for coordinate twisting. If the coordinate was `i64::MIN`, this caused an integer overflow panic, leading to a Denial of Service.
**Defense:** Replaced the subtraction with `.wrapping_sub()` to handle the overflow gracefully (preserving the modular arithmetic behavior).

## 2024-05-26 - Akashic Record Data Loss
**Threat:** The `AkashicWrite` OpCode would overwrite the entire database file if `load_records` failed (e.g. due to corruption or size limit), leading to catastrophic data loss.
**Defense:** Implemented atomic writes (write-to-temp + rename) and strict size checks in `save_records`. Refactored `load_records` to report errors instead of returning an empty map.

## 2025-01-27 - Locus Division by Zero Panic (DoS)
**Threat:** `Topology::normalize` panicked due to division by zero (via `rem_euclid`) when `width` or `height` were 0. This is a DoS vector if dimensions are user-controlled (e.g. terminal resize).
**Defense:** Added explicit checks for `width == 0 || height == 0` at the start of `normalize`, returning `None`.

## 2025-02-18 - Bio-Transit Grid Undefined Behavior
**Threat:** `TrailMap::diffuse_and_decay` used `unsafe { *self.grid.get_unchecked(...) }` inside a parallel loop. Since `TrailMap` fields are public, a user could truncate `grid` independently of `width` and `height`, causing the unchecked access to read out of bounds (UB).
**Defense:** Replaced the `unsafe` block with standard safe indexing. This turns the potential UB into a safe panic if invariants are violated.

## 2025-05-23 - Mobius Transformation Singularity (DoS)
**Threat:** The `Mobius` struct in `crates/poincare-disk` exposed public fields (`a`, `b`, `c`, `d`), allowing the construction of invalid transformations (e.g., zero determinant). Calling `apply` on such a struct would result in division by zero, propagating `NaN`/`Inf` throughout the simulation, potentially crashing or hanging downstream systems.
**Defense:** Made `Mobius` fields private and introduced a `new` constructor that validates the determinant is non-zero, returning `Option<Self>`. Added getters for read-only access.

## 2025-10-27 - Myco-Diffusion Grid Panic (DoS)
**Threat:** The `GrayScottGrid` struct in `experiments/myco-diffusion` exposed public fields (`width`, `height`, `u`, `v`), allowing external code to modify dimensions without resizing buffers. This inconsistency caused a panic (DoS) when `update` or `deposit_v` accessed the buffers using the modified dimensions.
**Defense:** Enforced encapsulation by making `GrayScottGrid` fields private and adding read-only accessors (`width()`, `height()`, `u()`, `v()`). This ensures the invariant `u.len() == width * height` is always maintained after initialization.

## 2026-02-16 - Nova Physics Integer Overflow Panic
**Threat:** The `exec_irradiate` and `exec_detox` OpCodes in `experiments/chimera-lang/src/vm/nova.rs` calculated energy cost using `(r * r + 1)` where `r` is a user-controlled `i64`. If `r` was large (e.g. `i64::MAX`), `r*r` would overflow `i64`, causing a panic in debug builds (DoS) or wrapping in release builds.
**Defense:** Switched to calculating `r_sq` using `(r as i128).saturating_mul(r as i128)` to ensure safety and correct cost capping.

## 2026-03-01 - Nova Strings Unbounded Allocation (DoS) & Retina Hardening
**Threat:** The `OpCode::StringNew` operation in `experiments/chimera-lang/src/vm/nova_strings.rs` allowed unbounded creation of `CosmicString` objects via an infinite loop, leading to memory exhaustion (DoS).
**Defense:** Introduced `MAX_STRINGS` (256) limit in `vm/mod.rs` and enforced it in `exec_string_op`.

**Threat:** `exec_rasterize` in `experiments/chimera-lang/src/vm/retina.rs` allowed integer underflow wrapping via `x as usize` when `x` was negative, potentially writing to index 0 instead of being bounds-checked.
**Defense:** Added explicit checks for `x < 0 || y < 0` in `exec_retina_draw` and `exec_rasterize`.

## 2026-04-10 - Simulation Amplification (DoS)
**Threat:** The `OpCode::Simulate`, `OpCode::Prophecy`, and `OpCode::Dream` operations in `experiments/chimera-lang` allowed recursive execution of the VM. By chaining these calls (Branching Factor > 1), a user could trigger exponential computational work ($Cost \approx Branching^{Depth}$) while only paying linear energy cost, causing a Denial of Service (CPU Exhaustion).
**Defense:** Introduced `MAX_SIMULATION_DEPTH` (10) constant and enforced it in `exec_simulate`, `exec_prophecy`, and `exec_dream`. This restricts the recursion depth of expensive simulation operations significantly compared to the standard `MAX_RECURSION_DEPTH` (100).

## 2026-05-20 - Unbounded DNA Strand Growth (OOM DoS)
**Threat:** `OpCode::BioHack` (Memetics), `Scavenge`, and `Digest` (Self-modification) allowed creating unbounded DNA strands by bypassing the `MAX_STRANDS` limit. A malicious genome could loop these instructions to consume infinite memory (OOM).
**Defense:** Added explicit checks for `vm.dna.helix.strands.len() >= MAX_STRANDS` in `memetics.rs` and `mod.rs`. Introduced `warden_security_oom.rs` regression test and fixed a flawed "red team" test (`havoc_poly_crash.rs`) that incorrectly panicked on success.

## 2026-02-18 - Unbounded Prologue Strands & GL Overflow
**Threat:** The `G` (Genesis) and `X` (Crossover) runes in `experiments/chimera-lang/src/vm/prologue/mod.rs` allowed creating unlimited DNA strands by bypassing the `MAX_STRANDS` limit. A malicious grid could trigger these runes in a loop to consume infinite memory (OOM DoS).
**Defense:** Added explicit checks for `vm.dna.helix.strands.len() < MAX_STRANDS` in `apply_sink_rune`. If the limit is reached, the operation is skipped and an error is logged.

**Threat:** `intersect_rect` in `experiments/chimera-tardis/src/safe_gl.rs` used standard addition `a.0 + a.2`, which could panic (debug) or wrap (release) if coordinates were large (e.g., `i32::MAX`), leading to incorrect rendering or DoS.
**Defense:** Replaced arithmetic with `.saturating_add()` and `.saturating_sub()` to ensure safe clamping behavior.

## 2025-05-27 - Nova Resource Exhaustion (DoS)
**Threat:** The `Broadcast` (Ether), `Reflex`, `Harmonize` (Chord Registry), and `TuiMod` (Event Queue) operations in `experiments/chimera-lang/src/vm/nova.rs` allowed unbounded allocation of resources via HashMap/Vec growth. A malicious program could loop these instructions to consume infinite memory (OOM DoS).
**Defense:** Introduced `MAX_ETHER_CHANNELS` (1024), `MAX_REFLEXES` (256), `MAX_CHORD_REGISTRY` (256), and `MAX_TUI_EVENTS` (64) constants in `vm/mod.rs` and enforced them in `vm/nova.rs`.

## 2026-06-15 - Unbounded Planes & Unsafe GL Scissor
**Threat:** The `OpCode::Dimension` and `OpCode::DWrite` operations in `experiments/chimera-lang/src/vm/nova_planes.rs` allowed creating an unbounded number of 2D planes (`vm.planes`), enabling memory exhaustion (DoS).
**Defense:** Introduced `MAX_PLANES` (64) constant in `vm/mod.rs` and enforced it in `exec_planes_op` and `OpCode::DWrite` logic. Verified with `warden_planes_dos_test.rs`.

**Threat:** `safe_gl::with_scissor` in `experiments/chimera-tardis/src/safe_gl.rs` passed user-controlled dimensions to `glScissor` without clamping when no parent scissor existed. Passing negative values is potentially unsafe/UB depending on driver behavior.
**Defense:** Added `.max(0)` clamping to width and height in `with_scissor` to ensure non-negative values are passed to OpenGL. Verified with `warden_gl_test.rs`.

## 2026-10-23 - Unbounded Resource Consumption (DoS)
**Threat:** Malicious Chimera programs could exhaust memory via unbounded creation of Memes (`OpCode::Conceive`), Viruses (`OpCode::Infect`), or Strands (`compile_cst`), or via unbounded Brainfuck output.
**Defense:** Introduced `MAX_MEMES` (64), `MAX_VIRUSES` (64), and `MAX_BRAINFUCK_OUTPUT` (1024). Enforced these limits in `memetics.rs`, `babel.rs` (checking `MAX_STRANDS` and recursion depth), and `nova.rs`. Added regression test `warden_resources_test.rs`.

## 2026-12-12 - Unbounded Gene Growth (OOM DoS)
**Threat:** `OpCode::Propagate` and `OpCode::Outbreak` (Transduction) in `experiments/chimera-lang` allowed appending genes to existing strands without checking for length limits. A malicious virus could repeatedly infect a strand, causing it to grow indefinitely until OOM.
**Defense:** Introduced `MAX_GENES_PER_STRAND` (4096) constant in `vm/mod.rs`. Enforced this limit in `memetics.rs` before extending any strand. Added `warden_gene_dos_test.rs` to verify the fix.

## 2026-12-13 - Alchemy Geometry Unbounded Allocation (OOM DoS)
**Threat:** The `OpCode::AbsorbGeometry` operation in `experiments/chimera-lang/src/vm/nova_alchemy_prime.rs` allowed unbounded allocation of a `Vec` via `Vec::with_capacity(count)` where `count` was derived from user-controlled `radius`. A malicious program could supply a large radius to trigger an OOM crash.
**Defense:** Implemented checked arithmetic for dimension calculations and enforced `count <= MAX_GENES_PER_STRAND` (4096) before allocation. Added `warden_alchemy_dos_test.rs` regression test.

## 2027-01-15 - Unbounded Fractal Iterations (DoS)
**Threat:** The `OpCode::Mandelbrot` and `OpCode::Escape` operations in `experiments/chimera-lang/src/vm/nova_fractal.rs` allowed setting `max_iter` to arbitrarily large values (e.g., `1_000_000_000`), causing the VM execution loop to hang the thread and causing a Denial of Service.
**Defense:** Introduced `MAX_FRACTAL_ITER` (1000) constant in `vm/mod.rs` and enforced it in `exec_fractal_op`. Renamed and updated `havoc_fractal_hang.rs` to `havoc_fractal_dos_prevention.rs` to verify the fix.

## 2027-02-23 - Nova Chemistry Integer Overflow Panic
**Threat:** The `exec_brew` operation in `experiments/chimera-lang/src/vm/nova_chemistry.rs` used unchecked addition (`+=`) to calculate potion potency. If the input heat or potency accumulated to `i64::MAX`, this would cause a panic in debug builds (DoS) or wrapping in release builds.
**Defense:** Replaced unchecked addition with `.saturating_add()` to ensure safety and prevent panic/wrapping. Verified with `warden_overflow_test.rs`.

## 2027-05-29 - Nova Fluid Wind Vector Overflow
**Threat:** The `process_fluid` function in `experiments/chimera-lang/src/vm/nova_fluid.rs` accumulated wind vectors using `i8` arithmetic. If multiple cells directed wind to a single target cell, the `i8` accumulator could overflow, causing a panic in debug builds (DoS) or wrapping behavior in release builds.
**Defense:** Promoted the wind vector accumulator to `(i32, i32)` to allow safe accumulation of contributions from all neighbors. The final result is clamped to `[-MAX_WIND, MAX_WIND]` and cast back to `i8` for storage, ensuring stability and preventing panic.

## 2027-06-19 - Linguistics Unbounded Memory Allocation (OOM DoS)
**Threat:** The `Levenshtein` OpCode and `≅` (Approx Equal) Rune in `experiments/chimera-lang` allocated memory proportional to $N \times M$ where $N$ and $M$ are string lengths. A malicious program providing large strings (e.g. 65k chars) could trigger massive allocations (e.g. 34GB), causing Out of Memory (OOM) DoS.
**Defense:** Introduced `MAX_COMPLEX_STRING_LEN` (1024) limit for $O(N^2)$ and $O(N \log N)$ linguistics operations. Optimized `levenshtein` implementation to use $O(\min(N, M))$ memory (2 rows) instead of $O(N \times M)$ matrix. Verified with `warden_linguistics_dos_test.rs`.

## 2027-06-25 - Ferrous Mycelium Race Condition
**Threat:** The `Hypha` struct in `experiments/ferrous-mycelium/src/organism.rs` used a `static mut ID_COUNTER` to generate unique IDs. Accessing `static mut` is `unsafe` and causes a data race in multi-threaded environments (UB), violating memory safety guarantees.
**Defense:** Replaced `static mut` with `std::sync::atomic::AtomicU64` and used `fetch_add(1, Ordering::Relaxed)` to safely generate unique IDs without `unsafe` blocks. Added `test_id_uniqueness` to verify behavior.

## 2027-07-15 - Evolution Engine Zero Population Panic
**Threat:** The `EvolutionEngine` in `experiments/chimera-lang/src/vm/evolution.rs` would panic with "index out of bounds" if initialized with a population size of 0 (via malicious configuration or direct API use). This occurred because `tournament_select` attempted to access an empty pool.
**Defense:** Updated `EvolutionEngine::new` to clamp `population_size` to `max(1)`, ensuring the population is never empty. Also hardened the parser in `compiler.rs` to correctly handle evolution properties using named rules, fixing a bug where configurations like `population: 0` were ignored (defaulting to 50) due to silent literal consumption in Pest.

## 2027-08-15 - Babel Live Parse Infinite Loop (DoS)
**Threat:** The inner loops in `nova_babel_live::exec_live_parse` (for string, regex, and action parsing) lacked iteration limits. A malicious grid configuration (e.g. toroidal wrapping without a terminating character) could cause an infinite loop, hanging the VM thread (DoS).
**Defense:** Added `loop_safety` counter to limit inner parsing loops to 256 iterations (matching `GRID_SIZE` squared). Verified with `warden_babel_dos.rs`.
## 2027-08-30 - Impossible Explorer Integer Overflow (DoS)
**Threat:** The `intersect_rect` function in `experiments/impossible-explorer/src/main.rs` used unchecked addition (`a.0 + a.2`) which could cause integer overflow when using large coordinates, leading to a panic in debug builds (DoS) or wrapping behavior in release builds.
**Defense:** Replaced unchecked addition and subtraction with `.saturating_add()` and `.saturating_sub()` to ensure safe coordinate clamping.

## 2027-09-10 - Platter Grid Encapsulation Failure (DoS)
**Threat:** The `Platter` struct in `crates/platter/src/lib.rs` exposed public fields (`width`, `height`, `magnetism`), allowing external code to modify dimensions without resizing the underlying vector. This inconsistency could cause a panic (DoS) or out-of-bounds reads/writes if `width` and `height` invariants were broken.
**Defense:** Enforced encapsulation by making `Platter` fields private and adding read-only accessor methods (`width()`, `height()`, `magnetism()`).

## 2027-10-15 - Unbounded File Read in Level Generation (OOM DoS)
**Threat:** The `generate_level` function in `experiments/heap-arena/src/level_gen.rs` used `fs::read_to_string` directly on files without any bounds checking. A maliciously crafted massive file could trigger an Out-of-Memory (OOM) Denial of Service (DoS) vulnerability by exhausting application memory.
**Defense:** Replaced the unbounded read with a capped reader using `std::io::Read::take(1024 * 1024)`. This guarantees that memory exhaustion attacks are thwarted by limiting parsing to the first 1MB of any input file. Added `test_generate_level_large_file_dos_prevention` to verify the safety.

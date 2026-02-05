# Warden's Journal 🔒

**2025-02-18 - Fugue State Panic & Karman Render Safety**
**Threat:** Denial of Service (DoS) via panic injection.
**Defense:** Replaced `unwrap()` on user input path in `fugue-state` and rendering bounds in `karman-text-street` with safe handling.

**2025-05-23 - Hyperbolic-FS Symlink Loop Hardening**
**Threat:** DoS via symlink loops in `hyperbolic-fs`, causing redundant processing or potentially unbounded recursion.
**Defense:** Implemented cycle detection in `scan_depth` using `canonicalize` and `ancestors` set.

**2026-02-03 - Quipu-Renderer UTF-8 Panic**
**Threat:** DoS via panic in `quipu-renderer` when parsing strings containing '≡'. Code attempted to slice at byte index 1 of a 3-byte character.
**Defense:** Removed unsafe slicing logic and ensured all string operations respect UTF-8 boundaries.

**2025-05-27 - Routing-Market Packet Panic**
**Threat:** DoS via panic in `experiments/routing-market` when `Network::tick` processes a packet with an invalid destination `NodeIndex`.
**Defense:** Added `self.graph.node_weight(p.dest).is_none()` check in `tick` to drop invalid packets, and fixed `burst` to generate valid indices.

**2025-05-28 - Alloc-Tardis Scissor Safety & Unwrap Removal**
**Threat:** Undefined Behavior via unchecked `unsafe` GL calls and DoS via `unwrap()` panic on invalid room IDs in `alloc-tardis`.
**Defense:** Encapsulated `glScissor` in RAII-guarded `with_scissor` wrapper and replaced panicking accessors with `if let Some(...)` checks.

**2026-02-19 - Chimera-Lang Integer Overflow Panic Hardening**
**Threat:** DoS via panic injection in `chimera-lang`. `get_circular_coords` allowed integer overflow on `r*r` and coordinate calculations. `diffuse_*` functions allowed integer overflow when summing grid values.
**Defense:** Upgraded `get_circular_coords` and `diffuse_*` arithmetic to use `i128` and `saturating_*` operations to prevent panics on extreme inputs.

**2026-03-01 - Chimera-Lang Resource DoS Hardening**
**Threat:** Denial of Service (DoS) via unbounded resource allocation (infinite Spore/Organelle creation) and call stack exhaustion (Reflex loops) in `chimera-lang`.
**Defense:** Enforced hard caps (`MAX_SPORES`, `MAX_ORGANELLES`, `MAX_CALL_STACK_DEPTH`) on resource vectors and reflex recursion.

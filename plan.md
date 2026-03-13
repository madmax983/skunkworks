1. **The Structural Mess (Tangle)**
   - `experiments/chimera-lang/src/vm/prologue/mod.rs` is almost 2,000 lines long.
   - It acts as both a module coordinator (with 72 `pub mod ...` declarations!) and a massive business logic file.
   - It contains a massive `process_agents` loop with a long `if/else if` chain switching over 20+ agent types (`"C"`, `"K"`, `"H"`, `"♻"`, `"♬"`, `"₣"`, `"Φ"`, `"ζ"`, `"P"`, `"⚓"`, `"∃"`, `"χ"`, `"🕷"`, etc.).
   - It contains methods like `pack_agent_data` and `unpack_agent_data` mixed with coordinate normalization and logic step loops (`exec_prologue_tick`).

2. **The Blueprint (Solution)**
   - Extract the agent registry and parsing logic (`process_agents`, `pack_agent_data`, `unpack_agent_data`, `PrologueAgent` struct) into a new `agent.rs` module.
   - Extract the state definition (`PrologueState`) and initialization (`impl Default for PrologueState`) into a new `state.rs` module.
   - Extract the tick execution pipeline (`exec_prologue_tick`, `prepare_signals`, `process_signal_propagation`, `process_reality_physics`, `process_sinks`, `normalize_coords`) into a new `engine.rs` module.
   - Convert `mod.rs` into a clean Facade that `pub use`s the extracted components and declares the 70+ submodules, shrinking it from ~2,000 lines to <200 lines.

3. **Verify**
   - Verify `cargo check -p chimera-lang` passes.
   - Verify `cargo test -p chimera-lang` passes.
   - Verify `cargo clippy -p chimera-lang -- -D warnings` passes.

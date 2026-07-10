1. **Evaluate Previous Hybrids (Phase 1):**
   - Check `MUTATIONS.md` for recently attempted crosses (e.g., `flock-platter`, `arthropod-market`, `arthropod-origami`, `arthropod-lattice`, `arthropod-flock`, `arthropod-physics`).
   - I have built and run `arthropod-physics`, `neuro-physics`, `arthropod-flock`, and `arthropod-lattice` confirming compilation, presence of headless execution modes where applicable, and noting execution/condemnation states. I will formalize this evaluation in `MUTATIONS.md` Phase 1 notes.

2. **Select and Design New Hybrid (Phase 2):**
   - Since the "Proposed Crosses" section is effectively empty or contains already-completed crosses, I will autonomously invent a new cross.
   - **Strategy Update**: Crossing discrete logical operations with continuous physical/biological simulation produces robust hybrid phenotypes.
   - **Selected Parents**: `crates/arthropod` (Immediate mode UI) and `crates/resonance-audio` (Real-time 2D wave physics).
   - **Proposed Hybrid**: `arthropod-resonance`.
   - **Novel Trait**: Mapping abstract immediate-mode GUI components onto an acoustic wave propagation grid, where button clicks and slider interactions act as active audio exciters or dynamic physical boundaries in the wave simulation.
   - **Predicted Phenotype**: An interactive synthesizer sandbox where manipulating GUI widgets visibly splashes waves across the 2D grid and alters acoustic standing wave patterns.

3. **Implementation:**
   - Create the directory `experiments/arthropod-resonance`.
   - Set up `Cargo.toml` linking the macroquad-based `arthropod`, `resonance-audio`, and `macroquad` dependencies.
   - Create `src/main.rs` that encapsulates the headless CI bypass for `macroquad` while tying the UI buttons to `AudioCommand::Pluck` inputs and wave visualizations.
   - Create a `README.md` containing the lineage and Quick Start instructions.

4. **Documentation and Traces:**
   - Pre-commit step to ensure documentation formatting.
   - Update `MUTATIONS.md` (Add the Phase 1 evaluation and add the new hybrid to Attempted Crosses).
   - Append a recombination pheromone trail to `GUESTBOOK.md`.
   - Commit with the message: `🧬 Splice: Cross arthropod × resonance-audio`.

Wait, `gray-fluid` is great.
"Concept: Magnetic Reaction-Diffusion. The magnetic particles (from ferrous-fluid) flow and swirl through a magnetic field. As they move, they deposit chemical 'V' onto the gray-scott grid. The concentration of chemical 'U' provides magnetic permeability, which amplifies the magnetic field's pull."

Let's write down the steps:
1. Create `experiments/gray-fluid` using `cargo new --bin experiments/gray-fluid`.
2. Add dependencies to `experiments/gray-fluid/Cargo.toml` (`locus`, `gray-scott`, `ferrous-core`, `tui-shared`, `ratatui`, `crossterm`, `anyhow`, `rand`).
3. Add `experiments/gray-fluid` to workspace `Cargo.toml`.
4. Copy `experiments/ferrous-fluid/src/*` to `gray-fluid`, modifying `physics.rs` and `main.rs` to include the `GrayScott` grid.
5. In the update loop:
   - Run the Gray-Scott reaction-diffusion step.
   - For each particle, calculate the magnetic pull towards the magnets. Multiply the magnetic pull by the 'U' chemical concentration at the particle's position.
   - For each particle, add some 'V' chemical to the Gray-Scott grid at its position (depositing).
   - Draw the particles, and maybe draw the Gray-Scott V chemical grid as a background (e.g., using block characters or colors).
6. Document lineage in `README.md`.
7. Update `MUTATIONS.md` to move/add the hybrid.
8. Update `GUESTBOOK.md` with recombination pheromone.
9. Pre-commit check + git commit.

Let's request a review.

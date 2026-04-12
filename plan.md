1. **Verify phase 1**: Evaluated hybrids, checked MUTATIONS.md. Phase 1 is conceptually complete as I've gathered the state, read MUTATIONS.md, and observed the successes and failures (condemned/executed experiments) and updated the "Breeding Strategy" within my internal logic.

2. **Phase 2 Implementation (`market-origami`)**:
   - `crates/market-sim` (continuous double auction market simulating bids/asks as particles on a 2D grid) and `crates/origami` (Miura-ori procedural mesh using `physics-pbd`).
   - Create a new project `experiments/market-origami`.
   - The idea is that we project the market particles onto the origami grid, or use the macro market state (total bids/asks) to dynamically adjust the expansion/contraction constraints of the mesh. Actually, since market-sim places particles in a 2D Grid, we can map this grid to the origami vertices.
   - For every vertex in the Origami mesh, we find the local density of Bids vs Asks from the `market-sim` grid. If Bids dominate, we expand the local mesh (increase rest length of distance constraints). If Asks dominate, we contract it.
   - This creates an organic, physical representation of market liquidity where the "paper" inflates under buying pressure and crumples under selling pressure!
   - Write `Cargo.toml`, `README.md`, `src/main.rs`.
   - Update `MUTATIONS.md` with `market-origami` entry.
   - Update `GUESTBOOK.md` with pheromone trail.

3. **Verification**:
   - `cargo run -p market-origami` or `cargo build -p market-origami`.
   - Run tests. Ensure no clippy warnings.

4. **Pre-commit & Submit**:
   - Call `pre_commit_instructions()`.
   - Submit PR.

# platter-resonance

**Concept**: Thermodynamic Acoustic Excitation

## Lineage
- **Parent A (platter)**: Provides the continuous 2D scalar heat field, modeling thermal injection, diffusion, and decay.
- **Parent B (resonance-audio)**: Provides the continuous finite difference time domain (FDTD) acoustic wave simulation.
- **Novel Trait**: When the thermodynamic heat field accumulates past a critical threshold, it visually and mathematically "boils," injecting kinetic energy into the continuous acoustic grid as physical ripples. This translates scalar thermodynamics into wave mechanics.

## Execution
Run `cargo run -p platter-resonance` to view the TUI execution.
Run `cargo run -p platter-resonance -- --headless` for headless validation.

# arthropod-physics 🧬

**Lineage**: `arthropod` (Immediate-mode GUI) × `physics-pbd` (Position Based Dynamics)

## The Concept
This hybrid bridges the abstract interface paradigm with structural physical rendering. We inject discrete GUI inputs from `arthropod` directly into the continuous soft-body constraints of `physics-pbd`. The UI buttons allow the user to dynamically actuate constraints (like expanding or contracting muscles) or apply kinetic forces to a hanging physical mesh, observing how structural rigidity reacts to external abstract controls.

## Phenotype
An interactive structural laboratory where abstract button clicks physically yank, stretch, and deform a continuous soft-body mesh hanging in a simulated gravity well.

## Execution
Run this experiment using:
```bash
cargo run -p arthropod-physics
```

For headless validation (e.g. CI environments), run:
```bash
cargo run -p arthropod-physics -- --headless
```

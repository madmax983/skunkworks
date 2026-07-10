# 🧬 arthropod-physics

**Lineage**: `arthropod` (Immediate Mode UI components) × `physics-pbd` (Position-Based Dynamics Soft Body Engine)

**Concept**: Interactive Structural Rigging.

**Novel Trait**: By crossing the immediate mode UI elements of `arthropod` directly with the continuous Position Based Dynamics constraints of `physics-pbd`, we construct a live interactive playground. The discrete button clicks interact directly with the physical constraints, exploding the continuous particle physics or adding dynamically connected structural chain elements.

**Phenotype**: A robust interactive structural playground showing continuous topological deformation of chains, where the abstract GUI directly influences the physical parameters of the environment.

**Status**: Attempted Cross

## Quick Start

```sh
# Run interactively (GUI)
cargo run -p arthropod-physics --release

# Run headlessly (CI/Non-interactive)
cargo run -p arthropod-physics --release -- --headless
```

# 🧬 Chimera Origami

**Lineage:** `experiments/chimera-lang` × `crates/origami`

This experiment demonstrates **Bio-Mechanical Genetics**. It bridges the discrete world of genetic programming with the continuous world of physical soft-body dynamics.

### 🧬 Parent A: `chimera-lang`
- Provides the `ChimeraVM` genetic execution engine.
- Supplies discrete opcode sequences (DNA) that govern the internal state ("brain") of the organism.

### 🧻 Parent B: `origami`
- Provides the continuous physical space via a Miura-ori mesh tessellation.
- Supplies the Position-Based Dynamics (PBD) constraints, specifically structural distance constraints and physical actuators.

### 🌿 Novel Trait: Kinetic Genetic Locomotion
Instead of acting on abstract variables or grid memory, the `ChimeraVM` directly controls the physical actuators within the Miura-ori mesh. The discrete execution of opcodes alters the stack, which determines whether the physical mesh actuators contract or relax. The result is a soft-body organism that twitches, folds, and locomotes through physical space purely driven by executing its genetic code.

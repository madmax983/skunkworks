# Chimera Biomorph 🧬

A hybridization of `chimera-lang` (Genetic Programming VM) and `biomorphic-lexicon` (Physics-based Strings).

## Concept

**Kinetic Mutagenesis**: The genetic code of a digital organism is visualized as a physical string of particles.
Each particle represents a Gene (OpCode). The string exists in a physics simulation with mass, tension, and damping.

When the string is agitated (by user interaction or high energy events), the kinetic energy exceeds a threshold, causing "mutations" in the genetic code. The physical stress literally rewrites the DNA.

## Lineage

- **Parent A**: `experiments/chimera-lang`
  - Source of the Genetic Code, Virtual Machine, and Execution Logic.
  - The "Mind" of the biomorph.

- **Parent B**: `experiments/biomorphic-lexicon`
  - Source of the Physics Engine (Mass-Spring System) and Rendering Logic.
  - The "Body" of the biomorph.

## Controls

- **Space**: Pluck the string (Inject Kinetic Energy).
- **Enter**: Reset the organism.
- **M**: Force a mutation immediately.
- **Q**: Quit.

## Emergent Traits

- **Physical Code**: You can see the structure of the program as a physical object. Heavy operations (IO, Math) have more mass than lighter ones (Flow Control).
- **Stress Evolution**: Programs that are "stable" (low kinetic energy) preserve their code. Programs that are "volatile" or agitated by the user mutate rapidly.

## Status

Compiles and runs. The VM executes a simple loop while the physics simulation runs in parallel.

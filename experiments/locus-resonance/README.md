# Topological Acoustic Morphogenesis (`locus-resonance`)

A hybrid organism born from the cross of `crates/locus` and `crates/resonance-audio`.

## 🧬 Lineage

- **Parent A (`locus`)**: Provides the topological coordinate mapping (Torus, Klein Bottle, Cylinder) that bends the concept of Euclidean spatial bounds.
- **Parent B (`resonance-audio`)**: Provides the discrete Finite Difference Time Domain (FDTD) wave physics simulation.

## 🔬 Phenotype

Instead of acoustic waves reflecting perfectly off the hard, rectangular boundaries of the 2D grid, the wave solver wraps the wave propagation calculations using `locus` topology. When an acoustic pressure wave travels off the "eastern" edge of the simulation, it wraps seamlessly to the "western" edge without losing energy. We observe standing waves and interference patterns forming in non-planar topologies (e.g., Torus, Klein Bottle), effectively creating a continuous topological acoustic wave tank.

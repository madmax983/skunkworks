# 🧶 Ferrous Weaver

> "The weaver knots the code, and the loom remembers the pattern."

**Ferrous Weaver** is a hybrid experiment that visualizes the "Genetic Code" of a ChimeraVM organism as a **Magnetic Quipu**.

## 🧬 Concept

1.  **Genome Generation**: Random `ChimeraVM` genomes (DNA strands) are generated.
2.  **Serialization**: The DNA is serialized into Incan Quipu cords using `quipu-serializer`.
    - `OpCodes` and Arguments become knots on a cord.
3.  **Physical Mapping**: The knots are mapped to physical bodies in a simulation.
    - **Simple Knot**: Positive Charge (+).
    - **Long Knot**: Negative Charge (-).
    - **Figure-Eight Knot**: Neutral (0).
    - Mass is determined by the knot type/value.
4.  **Magnetic Loom**: The cords hang in a physics simulation (`ferrous-quipu`) where:
    - They are subject to Gravity.
    - They interact via **Coulomb's Law** (Like charges repel, opposites attract).
    - They leave magnetic trails on a backing "Platter", creating a memory of their movement.

## 🔮 Emergence

By physicalizing the code structure into charged knots, similar genetic sequences should exhibit similar physical behaviors and potentially cluster together due to magnetic interactions. The "shape" of the code becomes the "shape" of the textile.

## 🕹️ Controls

- **`Q`**: Quit
- **`R`**: Regenerate Genome (New Random DNA)
- **`Up`/`Down`**: Move the "Playhead" (visualization line).

## 🧬 Lineage

- **Parent A**: `experiments/ferrous-quipu` (Physics, Magnetism)
- **Parent B**: `experiments/quipu-serializer` (Data -> Knot)
- **Source**: `experiments/chimera-lang` (DNA)

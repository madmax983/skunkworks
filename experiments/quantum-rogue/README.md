# Quantum Rogue ⚛️

A Roguelike where the dungeon is a Quantum System.

## Lineage
- **Parent A:** `experiments/market-rogue` (Grid movement, Roguelike structure)
- **Parent B:** `experiments/quantum-garden` (Quantum State Simulation, Gates)
- **Hybrid Vigor:** Dynamic Quantum System Management (merging/splitting systems).

## Concept
You are an observer in a quantum dungeon. The entities (Qubits) exist in superpositions of Empty (|0>) and Item (|1>).
- **Walk** into a Qubit to **Measure** it.
  - If it collapses to |1> (Red), you gain points.
  - If it collapses to |0> (Blue), it vanishes.
- **Inventory** contains Quantum Gates:
  - `H` (Hadamard): Put a Qubit into superposition (Magenta).
  - `X` (Pauli-X): Flip the state (NOT gate).
  - `C` (CNOT): Entangle two Qubits. Measuring one will affect the other!

## Controls
- `Arrow Keys`: Move / Measure
- `h`: Cast Hadamard Gate on nearest Qubit
- `x`: Cast Pauli-X Gate on nearest Qubit
- `c`: Cast CNOT (Entangle) on two nearest Qubits
- `q`: Quit

## Technical Details
The game manages multiple independent `QubitSystem`s. When you entangle two qubits from different systems, their state vectors are merged via tensor product into a larger system. When you measure a qubit, its system collapses and dissolves back into independent states (optimization).

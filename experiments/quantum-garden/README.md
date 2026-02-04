# Quantum Garden ⚛️

**"Where bits bloom and entanglement grows."**

Quantum Garden is a TUI-based quantum circuit simulator disguised as a garden. It visualizes the abstract concepts of quantum mechanics using organic metaphors.

## Concept

- **Plants (Qubits):** Each plant represents a qubit.
  - **Height:** Represents the probability of measuring the state |1> (bloom). A tall plant is almost certainly |1>, a seedling is |0>.
  - **Color:** Represents the phase of the quantum state (in a simplified mapping).
  - **Vines:** Represent entanglement between qubits.

- **Gardening Tools (Gates):**
  - **Water (Hadamard Gate):** Puts a plant into superposition (50% growth).
  - **Fertilizer (Pauli-X Gate):** Flips the state (Seed <-> Bloom).
  - **Pruning (Pauli-Z Gate):** Changes the phase without changing height.
  - **Entanglement (CNOT Gate):** Connects two plants. If the control plant blooms, the target plant flips.
  - **Harvest (Measurement):** Collapses the garden into a definite state (Seeds or Blooms), destroying superposition and entanglement.

## Controls

- **Arrow Keys:** Move selection cursor.
- **H:** Apply Hadamard Gate (Water).
- **X:** Apply Pauli-X Gate (Fertilize).
- **Z:** Apply Pauli-Z Gate (Prune).
- **C:** Select Control for Entanglement (then move to Target and press Enter).
- **M:** Measure (Harvest) the garden.
- **R:** Reset the garden.
- **Q / Esc:** Quit.

## Technical Details

- Uses `num-complex` for complex amplitude simulation.
- Implements a full state vector simulation (exponential scaling, limited to 5 qubits for TUI readability).
- Built with `ratatui` and `crossterm`.

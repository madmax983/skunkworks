# Quantum Boids ⚛️🐦

**"Spooky Action at a Distance... in a Flock."**

A hybrid experiment combining **Luminous Flock** (Boids) and **Quantum Garden** (Qubits).

## Concept

Each boid carries a **Qubit** state (`|ψ> = α|0> + β|1>`).
- **|0> (Blue):** The "Coherent" state. Boids prefer to flock and align.
- **|1> (Red):** The "Decoherent" state. Boids prefer to separate and scatter.
- **Superposition (Purple):** Boids exhibit mixed behavior.

## Mechanics

1.  **Entanglement:**
    - When boids come within close proximity, they may become **Entangled**.
    - Entangled pairs (connected by gray lines) share a quantum bond.
    - **Spooky Action:** If one boid's state changes (due to measurement or interaction), it instantaneously affects its partner via a simulated CNOT gate.

2.  **Quantum Flocking:**
    - The boid's behavior weights (Alignment vs Separation) are modulated by the probability of measuring |1> (`Prob(|1>)`).
    - High `Prob(|1>)` -> Stronger Separation (Scattering).
    - Low `Prob(|1>)` -> Stronger Cohesion (Gathering).

3.  **Measurement:**
    - Pressing **'m'** collapses the wave function of all boids.
    - Each boid is forced into |0> or |1> based on its probability.
    - Entanglement is broken upon measurement.

## Controls

- **'q'**: Quit
- **'r'**: Reset the simulation
- **'m'**: Measure (collapse all wave functions)

## Lineage

- **Parent A:** `experiments/luminous-flock` (Boid physics, TUI)
- **Parent B:** `experiments/quantum-garden` (Qubit logic, Complex numbers)
- **Novel Trait:** Quantum Entanglement applied to swarm intelligence.

## License

MIT

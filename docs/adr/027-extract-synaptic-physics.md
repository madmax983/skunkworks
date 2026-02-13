# 027. Synaptic Physics Library

## Status
Accepted

## Context
Experiments involving neural networks or brain-inspired computing (like `neuro-terminal` or `synaptic-physics`) require a mathematical model for neurons. Implementing these models from scratch repeatedly leads to inconsistencies, potential bugs, and difficulty in comparing results across experiments. The Izhikevich model is a popular choice due to its balance of biological plausibility and computational efficiency, but its parameters and update logic are non-trivial to implement correctly every time.

## Decision
We will encapsulate the Izhikevich neuron model and related synaptic physics logic into a dedicated crate: `crates/synaptic-physics`.

### Key Components:
1.  **Izhikevich Struct:** Holds the state (`v`, `u`) and parameters (`a`, `b`, `c`, `d`) of a single neuron.
2.  **Update Loop:** Implements the numerical integration (Euler method) for the differential equations governing membrane potential.
3.  **Parameter Presets:** Provides constructors for common neuron types (Regular Spiking, Fast Spiking, Chattering) and randomization logic.
4.  **Current Injection:** Allows for external current to be injected into the neuron, simulating synaptic inputs.

## Consequences
### Positive
*   **Reuse:** The complex Izhikevich equations are implemented once and reused across all neural experiments.
*   **Correctness:** Centralized implementation ensures that the model behaves consistently and correctly according to the original paper.
*   **Performance:** The library can be optimized for performance (e.g., using SIMD or efficient memory layouts) benefiting all consumers.

### Negative
*   **Flexibility:** While Izhikevich is versatile, users might need other models (e.g., Hodgkin-Huxley or Integrate-and-Fire) which would require extending the library or creating new crates.

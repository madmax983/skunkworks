# Neuro Speech 🧠🗣️

> "The action potential drives the plosive." - The Splice Surgeon

**Neuro Speech** is a hybrid experiment simulating the neuromuscular control of speech. It combines the particle physics of `morph-physics` with the biophysical neuron simulation of `biophysical-synth`.

## Concept

A "Brain" composed of **Hodgkin-Huxley neurons** controls a "Vocal Tract" composed of **Phoneme Particles**.
- **Neurons**: Simulated with realistic ion channel dynamics (Na, K, Leak).
- **Muscles**: Neurons are connected to particles via "Muscles" (Axons). When a neuron spikes (voltage > -50mV), it exerts a force on its target particle.
- **Phoneme Particles**: Have physical properties (Place, Manner, Voice) and are connected by springs to form words.

## Lineage

*   **Parent A**: `experiments/morph-physics`
    *   Allele: Phoneme features (Place, Manner, Voice), Particle Physics Engine.
*   **Parent B**: `experiments/biophysical-synth`
    *   Allele: Hodgkin-Huxley neuron model (`neuron.rs`).
*   **Novel Trait**: **Neuromuscular Phonology**. Speech is not a string manipulation, but a physical act driven by neural spikes.

## Controls

*   **[1-5]**: Stimulate "Levitator" neurons (Pull particles UP).
*   **[Q-T]**: Stimulate "Gravitator" neurons (Pull particles DOWN).
*   **[SPACE]**: **Babbling Mode**. Randomly stimulate neurons to produce emergent phonetic movement.

## Running

```bash
cargo run -p neuro-speech
```

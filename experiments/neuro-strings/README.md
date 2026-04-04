# Neuro Strings 🎻🧠

**Lineage:** `crates/neuro-sim` (Spiking Neural Network) × `experiments/ferrous-strings` (Acoustic-Magnetic Physics).

## Concept
A set of vibrating strings that generate magnetic fields. A Spiking Neural Network (SNN) controls the strings and listens to the environment.

The system is a bidirectional bio-acoustic feedback loop:
1. **Brain to Matter (Motor Output):** When specific neurons spike, they physically "pluck" the strings, creating continuous acoustic vibrations and magnetic fields.
2. **Matter to Brain (Sensory Input):** The strings magnetize a continuous fluid `Platter`. The total magnetic flux acts as an input current to the neural network's sensory neurons.
3. **Emergence:** The discrete neural spikes generate continuous magnetic waves, and the magnetic waves dictate the future firing rate of the network. This creates dynamic, self-sustaining rhythmic loops.

## Controls
- **Mouse Click:** Stimulate the sensory neuron manually.
- **Audio:** Enabled if `cpal` is present (default).

## Emergent Phenotype
- **Bio-Acoustic Rhythm:** The network naturally discovers stable rhythmic patterns where the delayed magnetic feedback reinforces the firing sequence.
- **Chord Generation:** Multiple neurons can synchronize their firing, plucking different strings simultaneously to produce chords.

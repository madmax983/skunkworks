# Quantum Lexicon 👁️⚛️📜

**A Hybrid of `retinal-chaos` and `hyperbolic-lexicon`.**

This experiment explores the "Observer Effect" in a linguistic context. Words drift and mutate in an infinite hyperbolic plane, but the act of observing them (via a simulated retina) stabilizes their form.

## Concept
- **Hyperbolic Space**: Words exist on a Poincaré Disk, where space is infinite but bounded in the unit circle.
- **Phonetic Drift**: Without observation, words drift in space and their phonemes mutate randomly (e.g., vowels shift, consonants soften).
- **The Retina**: A simulated biological retina (Izhikevich neurons) watches the screen.
- **Quantum Zeno Effect**: High neural activity (spikes) creates a "Stability Field". When you look at a word (center it in the view), the retina fires, and the word freezes in place and form.

## Lineage
- **Parent A**: `experiments/retinal-chaos` - Provided the `Retina` simulation and Izhikevich neuron model.
- **Parent B**: `experiments/hyperbolic-lexicon` - Provided the Poincaré Disk rendering, particle system, and phonological features.
- **Novel Trait**: **Observation-Dependent Reality**. The simulation reacts to the simulated "gaze".

## Controls
- **WASD**: Move the view (Hyperbolic Translation).
- **Mouse**: The retina watches the screen content.
- **Visuals**:
  - **Background**: Hyperbolic tiling.
  - **Text**: Drifting words. Green = Stabilized (Observed). Colored = Drifting/Mutating.
  - **Overlay**: White flashes indicate retinal ganglion spikes.

## Technical Details
- **Rendering**: Macroquad with custom GLSL shaders for hyperbolic geometry.
- **Simulation**:
  - **Retina**: 128x128 grid of Photoreceptors -> Horizontal Cells -> Bipolar Cells -> Ganglion Cells.
  - **Physics**: Brownian motion on the complex plane (Poincaré model).
  - **Linguistics**: Phonemes are vectors of features (Voice, Place, Manner). Mutation adds noise to these vectors.

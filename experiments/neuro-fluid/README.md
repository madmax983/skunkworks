# 🧠💧 Neuro-Fluid

A biological-magnetic fluid hybrid experiment.

## 🧬 Lineage

**Parents**:
- `crates/neuro-sim` (Spiking Neural Network using the Izhikevich model)
- `experiments/ferrous-fluid` (Magnetic particle fluid simulation)

**Created by**: The Splice Surgeon 🧬

## 🔬 Phenotype (Emergent Trait)

**Brain-Fluid Interface:**
This experiment replaces the static user-controlled magnets of `ferrous-fluid` with dynamic "electromagnets" controlled by a Central Pattern Generator (CPG) built with `neuro-sim`.
The CPG consists of a ring of neurons connected by excitatory synapses.
When a neuron spikes, its corresponding electromagnet pulses with a massive burst of force.
The resulting emergent behavior is a rhythmic, pulsating fluid simulation driven entirely by the biological oscillations of the underlying Spiking Neural Network. It creates complex, breathing fluid patterns driven by neural oscillations.

## 🎮 Controls

- `[SPACE]` Inject Chaos (Stimulate all neurons to spike)
- `[R]` Reset Simulation
- `[Q] / [Esc]` Quit

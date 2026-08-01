# arthropod-neuro 🐜🧠

**Hybrid:** `arthropod` × `neuro-sim`

## Lineage
- **arthropod:** Provides the immediate-mode UI framework (`Button`), enabling interactive injection of events.
- **neuro-sim:** Provides the biological Spiking Neural Network (SNN) engine, evaluating Izhikevich neuron potentials continuously.

## Phenotype
This hybrid allows a user to interactively "poke" biological neurons via a UI button, injecting instantaneous current into a continuous physics/biology simulation. This cross bridges the discrete world of GUI abstractions with the continuous wave propagation of biological thoughts.

## Execution
```bash
cargo run -p arthropod-neuro
cargo run -p arthropod-neuro -- --headless
```

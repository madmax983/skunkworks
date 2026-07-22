# Arthropod × Neuro Sim (`arthropod-neuro`)

A biological interactive UI hybrid.

## Lineage
- **Parent A (`arthropod`):** Provides the interactive immediate-mode `Button` UI widget, capturing discrete mouse clicks.
- **Parent B (`neuro-sim`):** Provides the continuous biological simulation of an `Izhikevich` spiking neuron.
- **Novel Trait:** The discrete UI clicks inject physical current ($I$) directly into the continuous biological model, translating human UI interaction into artificial neural spikes and mapping the membrane potential ($v$) back to the UI visual space.

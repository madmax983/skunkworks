# Neuro-Flock

A hybrid experiment crossing `crates/neuro-sim` and `crates/flocking`.

## Lineage
- **From `flocking`**: Craig Reynolds' Boids algorithm managing separation, alignment, and cohesion of continuous swarm agents.
- **From `neuro-sim`**: A discrete Spiking Neural Network (SNN) based on the Izhikevich model.

## Novel Trait
**Neural Swarming**: The continuous swarming boids act as sensory input to the embedded SNN. In turn, the discrete cascading spikes of the neural network dynamically modulate the global flocking parameters. High neural activity leads to highly aligned, panicked swarming, while low neural activity leads to relaxed, separated foraging behavior.

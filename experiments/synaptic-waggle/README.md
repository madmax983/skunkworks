# Synaptic Waggle 🐝🧠

A hybrid experiment combining swarm intelligence with spiking neural networks.

## Lineage
- **Parent A**: `waggle-dance` (Swarm Consensus + Honeybee Communication)
- **Parent B**: `biomimetic-synth` (Izhikevich Neurons + STDP)
- **Concept**: A "Beuron" (Bee-Neuron). The hive is a distributed, mobile neural network.

## Concept
In this simulation, each bee is an autonomous agent carrying an Izhikevich neuron model.
- **Wandering**: The bee moves randomly (Brownian motion), representing the diffusion of neurotransmitters or the search for input.
- **Stimulus**: Green circles represent external input current. Bees inside these zones receive excitation.
- **Spiking (Dancing)**: When a bee's membrane potential ($v$) crosses the threshold ($30mV$), it stops and performs a **Waggle Dance**.
- **Synapse**: The dance emits a signal. Nearby bees (within the broadcast radius) receive synaptic input current, increasing their own potential.

This creates emergent waves of synchronization. A strong stimulus triggers a cluster of dances, which recruits nearby bees, propagating the "thought" through the swarm.

## Controls
- **Left Click**: Move the primary stimulus source.
- **Space**: Add a new stimulus source at the mouse position.
- **C**: Clear all stimulus sources.
- **R**: Reset the simulation.

## Technical Details
- **Neuron Model**: Izhikevich (Regular Spiking / Chattering).
- **Integration**: Euler method.
- **Swarm Physics**: Simple bounded Brownian motion.
- **Rendering**: Macroquad.

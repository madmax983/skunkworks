# Neuro-Syncopation 🧠🥁

> "The brain's rhythm is physically constrained by the OS scheduler."

A hybrid experiment combining the parallel threading model of `syncopated-threads` with the biological neuron simulation of `neuro-sim`.

## Concept

In a biological brain, neurons operate in parallel. In most simulations, we update them sequentially in a loop.
**Neuro-Syncopation** breaks this convention:

*   **1 Neuron = 1 OS Thread**
*   **Synapse = Mutex**

When a neuron spikes, it must acquire a lock on the downstream neuron's input buffer to deposit charge. If the target is busy (processing its own update or receiving another spike), the sender blocks.

This introduces **Parallel Neural Dynamics**: the firing rate and synchronization of the network are not just functions of the biological model (Izhikevich), but also of the hardware's ability to schedule threads and manage lock contention.

## Lineage

*   **Parent A:** `experiments/syncopated-threads` (Rhythm via Thread Contention)
*   **Parent B:** `crates/neuro-sim` (Izhikevich Neuron Model)

## Controls

*   `q`: Quit
*   `r`: Randomize Network

# Neuro Beats 🧠🎶

A "Moonshot" experiment combining **Neural Oscillations** and **Music Generation**.

This experiment simulates a network of Izhikevich neurons connected in a ring topology. Each neuron is assigned a frequency from a pentatonic scale. When a neuron spikes, it triggers a sound.

The result is a self-organizing musical pattern generator. The network's synaptic delays and weights create rhythmic loops.

## Features

*   **Biologically Plausible Neurons**: Uses the Izhikevich model (via `synaptic-physics`).
*   **Audio Synthesis**: Real-time sine wave synthesis using `rodio`.
*   **Visualization**: Real-time spike raster and network visualization using `macroquad`.
*   **Interactivity**: Click neurons to inject current and disturb the rhythm.

## Controls

*   **Left Click**: Stimulate the nearest neuron (injects current).
*   **SPACE**: Stimulate all neurons simultaneously (reset/sync).

## How it works

1.  Neurons are arranged in a ring.
2.  Each neuron connects to its neighbor with a delay.
3.  When a neuron spikes, it sends a current pulse to its neighbor.
4.  If the pulse is strong enough, the neighbor spikes after a delay.
5.  This creates a traveling wave of activity.
6.  Cross-connections add complexity and polyrhythms.

## Dependencies

*   `macroquad`
*   `rodio`
*   `synaptic-physics` (local crate)

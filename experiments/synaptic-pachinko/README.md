# Synaptic Pachinko

**Parents**: `packet-pachinko` × `synaptic-choir`

A hybrid experiment combining physics-based pachinko mechanics with a spiking neural network audio synthesizer.

## Concept
- **Visuals**: A TUI Pachinko board where packets (HTTP, SSH, Malware) fall through a grid of pins.
- **Physics**: Pins are physical obstacles that bounce packets.
- **Audio/Biology**: Each pin is an Izhikevich neuron. When a packet hits a pin, it injects current into the neuron.
- **Emergence**: The network produces a soundscape based on the flow of packets. Neurons light up when excited (Yellow) or spiking (Red/White).

## Controls
- `Space`: Drop a random packet.
- `Q` / `Esc`: Quit.

## Lineage
- `packet-pachinko`: Provided the TUI structure and physics engine.
- `synaptic-choir`: Provided the Izhikevich neuron model and audio engine (cpal).

# Locust Diffusion

**Experiment Type:** Swarm Intelligence / Reaction-Diffusion / Cybersecurity Visualization
**Status:** [FRESH]
**Stack:** Rust, Macroquad, Rayon

## Concept

Reaction-Diffusion Cyberwarfare. The server emits the "V" chemical (catalyst), which diffuses outward. The firewalls emit "U" chemical (inhibitor). The DDoS packets (locusts) treat the chemical gradients as continuous scalar fields for navigation, seeking high V while avoiding U. Furthermore, when packets die, they deposit localized pulses of U, causing the Turing patterns to morph and shift defensively against the attack.

The attack waves naturally reshape the chemical environment, creating organic, pulsating labyrinthine pathways that the packets must continuously solve to reach the server.

## Controls

- **Left Click:** Deploy a Firewall (Pesticide zone).
- **C:** Clear all Firewalls.

## Technical Details

- **100,000 Agents:** Simulated in parallel using `rayon`.
- **Render Buffer:** Direct pixel manipulation for high-performance visualization of the swarm.
- **Gray-Scott System:** Reaction-diffusion backend guiding swarm navigation.

# Origami Swarm 🦢

**A hybrid of `neuro-fold` and `luminous-flock`.**

"The flock breathes."

## Concept

This experiment combines **Position Based Dynamics (PBD)** soft-body simulation with **Boid Flocking** logic.
Each agent is a physically simulated "Origami Bird" (a simple hinged mesh) driven by a **Spiking Neural Network (CPG)**.

-   **Body**: A 4-particle mesh with structural constraints and an "Actuator" muscle.
-   **Brain**: A 2-neuron Central Pattern Generator (CPG) that drives the actuator to flap the wings.
-   **Flocking**: Standard Reynolds boid rules (Separation, Alignment, Cohesion) applied to the center of mass.
-   **Synchronization**: A Firefly/Kuramoto phase coupling mechanism synchronizes the CPGs of the flock, causing them to flap in unison.

## Lineage

-   **Parent A**: `experiments/neuro-fold` (The Splice Surgeon) - Provided the PBD physics engine and Izhikevich Neural Network code.
-   **Parent B**: `experiments/luminous-flock` (The Splice Surgeon) - Provided the Flocking logic and Phase Synchronization concept.

## Emergent Behavior

-   **Synchronized Flight**: The flock naturally synchronizes its wing beats due to phase coupling.
-   **Soft-Body Dynamics**: The birds deform and react to forces, giving them an organic feel.
-   **Visual Pulse**: The birds pulse in brightness based on their neural phase.

## Controls

-   **Mouse**: Camera control (Orbit).
-   **Visuals**: Brightness indicates neural phase.

## Build

```bash
cargo run -p origami-swarm
```

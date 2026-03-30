# 🧬 chaos-fluid

A hybrid experiment spliced from `chaos-pendulum` and `ferrous-fluid`.

## Concept
Chaotic Magnetic Fluid. A chaotic pendulum's tip deposits magnetic force into a fluid simulation.

## Novel Trait
Chaotic Swarm Manipulation. The pendulum acts as an unpredictable strange attractor pulling the ferrous particles into chaotic, shifting shapes.

## Lineage
- **From `chaos-pendulum`:** The chaotic double pendulum physics system, including calculations for theta, omega, and the resulting physical coordinates of the pendulum tip.
- **From `ferrous-fluid`:** The `ferrous_core::Platter` magnetic field simulation and the `MagneticParticle` swarm mechanics that react to magnetic field gradients.
- **Emergent Trait:** The pendulum directly deposits magnetic force into the platter instead of just swinging. The swarm attempts to align to the shifting strange attractor, leading to fluid-dynamic bottlenecking and stretching as the attractor sweeps across the domain.

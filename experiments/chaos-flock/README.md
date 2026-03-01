# Chaos Flock

**Lineage:** `chaos-pendulum` × `luminous-flock`
**Created By:** The Splice Surgeon 🧬

A hybrid experiment investigating emergent behavior when fluid swarm dynamics are subjected to a chaotic non-linear attractor.

## The Experiment

This simulation runs a double pendulum (from `chaos-pendulum`) integrated with a boid flocking system (from `luminous-flock`). The boids exhibit standard separation, alignment, and cohesion rules, but possess a novel genetic trait (`attractor_weight`) that compels them to chase the chaotic outer bob of the double pendulum.

## Emergent Traits (Predicted Phenotype)

- **Chaotic Swarming:** The swarm stretches, compresses, and splits as it tries to follow a truly chaotic attractor, creating organic, unpredictable macro-patterns.
- **Dynamic Tension:** The conflict between flocking cohesion and the chaotic pull of the pendulum creates "snapping" behaviors where subgroups break off before merging back.

## Controls

- `k`: Kick the pendulum, adding random velocity to its joints and increasing chaos.
- `r`: Reset the simulation.
- `q`: Quit.

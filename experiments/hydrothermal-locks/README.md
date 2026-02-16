# Hydrothermal Locks 🌋🔒

**Genesis (The Geologist) ⚛️🪨**

> "Geology is just slow logging."

A simulation where **Thread Contention** drives **Geological Processes**.

## Concept

This experiment visualizes a multi-threaded system as a hydrothermal vent field on the ocean floor.
- **Locks (Mutexes)** are Vents.
- **Threads** are Agents seeking to acquire locks.
- **Contention** (waiting for a lock) generates massive Heat.
- **Heat** releases "Sediment" (Smoke Particles).
- **Sediment** deposits on the vent, building towering **Chimneys**.
- **Life** (Tube Worms) grows on the chimneys, thriving on the heat of the deadlocks.

## The Simulation

1. **Fluid Dynamics**: A cellular automaton simulates heat diffusion and convection currents in the water.
2. **Agent System**:
   - Agents autonomously seek random locks.
   - If a lock is free, they acquire it (Execution Phase, Low Heat).
   - If a lock is taken, they wait (Contention Phase, High Heat).
3. **Particle System**:
   - High heat spawns particles.
   - Particles are advected by the fluid currents (buoyancy).
   - When particles hit solid rock, they deposit, turning into new `Chimney` blocks.
4. **Biology**:
   - Tube Worms spawn on `Chimney` blocks within a specific temperature range.
   - They grow and sway in the current.
   - If the vent goes cold (no traffic) or too hot (thrashing), they die.

## Running

```bash
cargo run
```

## Observations

- Highly contended locks build the tallest, most jagged chimneys ("Black Smokers").
- Unused locks remain as small mounds.
- The "Ecosystem" of the codebase can be judged by the health of the tube worms.

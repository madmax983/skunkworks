# Hydro Soundscapes 🧬

**Parents**: `experiments/hydro-flock` × `experiments/biochemical-soundscapes`

A visualization where a flock of boids interacts with a reaction-diffusion chemical field, creating an emergent soundscape.

## Concept
- **The Flock**: Boids emit a chemical activator (Chemical B) as they move. They are attracted to high concentrations of this chemical, creating feedback loops where they follow the trails they create.
- **The Field**: A grid simulates a reaction-diffusion system (Gray-Scott variant). Chemical A (Substrate) is omnipresent but consumed. Chemical B (Activator) diffuses and reacts.
- **The Sound**: The total concentration of chemicals drives an FM synthesis engine.
  - **Chemical A** controls the base pitch (Energy level).
  - **Chemical B** controls the modulation depth (Chaos/Timbre).

## Emergent Behavior
The flock "paints" the sound. As boids cluster, they create intense spots of Chemical B, increasing the modulation complexity of the audio. When they disperse, the sound becomes purer. The fluid dynamics (reaction-diffusion) create shifting patterns that the boids navigate, forming a bio-mechanical loop.

## Controls
- **Visuals**:
  - **Blue/Cyan**: Chemical A (Substrate).
  - **Red/Yellow**: Chemical B (Activator/Pheromone).
  - **White Triangles**: Boids.

## Lineage
- `hydro-flock`: Provided the boid flocking and fluid grid infrastructure.
- `biochemical-soundscapes`: Provided the reaction-diffusion concept and the audio synthesis engine.

## Usage
```bash
cargo run --release -p hydro-soundscapes --features audio
```
(Audio feature is optional and requires `cpal` dependencies).

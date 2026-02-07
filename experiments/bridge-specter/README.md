# Bridge Specter

**Bio-acoustic Hydro-Engineering**

An experiment combining swarm intelligence with fluid dynamics and audio reactivity. Army ants attempt to build bridges across a fluid medium that is constantly disrupted by sound waves.

## 🧬 Lineage

This experiment is a splice of two parents:

- **Parent A**: `experiments/biomimetic-bridge` (Genesis: The Entomologist)
  - *Traits Inherited*: Ant colony simulation, dynamic bridge formation logic, pheromone trails.
- **Parent B**: `experiments/fluid-specter` (Genesis: The Synesthete)
  - *Traits Inherited*: Real-time fluid simulation (Stable Fluids), audio spectrum analysis (FFT), ghost mode.

## 🧪 Emergent Behavior

- **Hydro-dynamic Stress**: Loud audio (especially bass) creates waves in the fluid, increasing local density and velocity.
- **Structural Resilience**: Ants treat high-density fluid as "gaps" and attempt to bridge them. However, strong waves can sweep ants away or break existing bridges (Panic state).
- **The Sound of Struggle**: The visualizer shows the colony fighting against the music to maintain connectivity.

## 🎮 Controls

- **Left Click**: Disturb fluid (add density/velocity).
- **Right Click**: Spawn more ants.
- **Audio**: Reacts to system microphone (if enabled) or generates "Ghost Mode" synthetic beats.

## 📦 Run

```bash
cargo run -p bridge-specter --release
```
To enable real audio input:
```bash
cargo run -p bridge-specter --features audio --release
```

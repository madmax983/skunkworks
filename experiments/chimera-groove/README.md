# Chimera Groove 🧬🎵

**Lineage**: `chimera-lang` (Parent A) x `atomic-groove` (Parent B)

## Concept
**"Rhythmic Genetic Programming"**

In this experiment, the execution of the Chimera Virtual Machine is synchronized with a global metronome. The environment pulses with time, injecting the current "Groove Phase" (0-100%) into the grid at coordinates `(0,0)`.

Critters (agents) must evolve to read this phase and time their actions to the beat. Actions performed "on beat" (>90% phase) are rewarded (e.g., energy gain via `photosynthesize`), while off-beat actions are costly.

## Novel Trait
**Phase-Locked Evolution**: The ability of genetic code to sense and synchronize with an external temporal rhythm.

## How to Run
```bash
cargo run -p chimera-groove
```

## Output
- TUI Visualization of the "Groove Bar" and the VM Grid.
- `chimera_groove_session.wav`: A generated audio file of the session's rhythm.

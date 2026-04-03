# Chimera Resonance 🧬🔔

**"The Song of the Genome"**

## Lineage
- **Parent A**: `experiments/resonance-chamber` (Physics-based Audio Synthesis)
- **Parent B**: `experiments/chimera-lang` (Biological VM)

## Concept
A population of virtual organisms (`ChimeraVM` agents) inhabits a 2D wave equation grid. As they execute their genetic code (DNA), they perform actions that mechanically "pluck" the grid at their location. These physical disturbances propagate as waves and are sonified in real-time.

The result is a generative soundscape driven by metabolic activity. A "Plucker" organism might pluck rhythmically. A "Wanderer" might create doppler-like effects. A "Colony" creates complex interference patterns.

## Mechanism
- **Environment**: A 60x30 grid solving the 2D wave equation.
- **Agents**: Each agent contains a running `ChimeraVM` instance.
- **Interaction**:
  - Agents execute `OpCode`s.
  - If the stack top contains a positive integer (e.g., via `PUSH 50`), it is interpreted as a "Pluck" command with strength proportional to the value.
  - The value is popped, and the grid is excited at the agent's (x, y) coordinates.
  - Agents can also move (consuming energy) or evolve (mutating DNA).

## Running
To hear the resonance (requires ALSA/cpal support):
```bash
cargo run -p chimera-resonance --features audio
```

To run visual-only (fallback):
```bash
cargo run -p chimera-resonance
```

## Controls
- `q`: Quit

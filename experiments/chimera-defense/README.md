# Chimera Defense 🧬🏰

**Hybrid Experiment**: `chaotic-defense` + `chimera-lang`

A Tower Defense game where the towers are autonomous agents driven by **ChimeraVM**.

## Concept

In a world governed by deterministic chaos (Logistic Map), you must build defenses. However, these are not ordinary turrets. Each tower contains a biological computer (ChimeraVM) that must calculate firing solutions.

- **Chaos**: Enemy spawn rates and movement jitter are driven by $x_{n+1} = r \cdot x_n \cdot (1 - x_n)$.
- **Biology**: Towers are programmable organisms. They sense the environment via their memory grid and actuate weapons by writing to specific memory cells.

## Controls

- `WASD`: Move cursor.
- `Space`: Build Tower (Cost: 20 Resources).
- `Left/Right`: Adjust Global Chaos ($r$). High $r$ means unpredictable spawns but potentially faster resource generation (via kills).
- `Q`: Quit.

## Tower Architecture

Each Tower runs a ChimeraVM instance.

### Sensors (Inputs)
The world writes sensor data to the VM's Grid before every tick:
- `Grid[0][0]`: Distance to nearest enemy.
- `Grid[0][1]`: Angle to nearest enemy (scaled x100).

### Actuators (Outputs)
The world reads the VM's Grid after execution:
- `Grid[15][0]`: Fire Command. If > 0, the tower fires a projectile.

### Default DNA
The default tower strain runs a simple loop:
1. Push `1` (Fire Signal)
2. Write to `Grid[15][0]`
3. Loop

Future versions will allow players to splice DNA from successful towers to evolve better targeting algorithms.

## Lineage

- **Parent A**: `experiments/chaotic-defense` (The engine, the chaos).
- **Parent B**: `experiments/chimera-lang` (The brain, the genetics).
- **Novelty**: Programmable, evolving defenses against chaotic threats.

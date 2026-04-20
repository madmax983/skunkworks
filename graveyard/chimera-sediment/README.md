# 🍂 Chimera Sediment

**Necrotic OS Visualization**

> "The forest is alive, but the soil is made of the dead."

## 🧬 Lineage
- **Parent A**: `experiments/chimera-canopy` (Process Trees, Living Processes)
- **Parent B**: `experiments/primordial-sediment` (Decay, Detritivores)
- **Concept**: An ecosystem where active processes are Trees (Canopy) and dead processes (zombies/terminated) fall as leaves to the forest floor (Sediment), where "Detritivore" agents (ChimeraVM) break them down.

## 🌿 Overview
This experiment visualizes the Operating System's process lifecycle as a biological cycle of growth and decay.
1. **The Canopy**: Active processes grow as procedural trees. Height = PID, Branching = Memory, CPU = Sunlight.
2. **The Fall**: When a process dies (disappears from `sysinfo`), it "collapses" into `SedimentParticle`s.
3. **The Sediment**: Dead code/process debris piles up on the forest floor.
4. **The Detritivores**: ChimeraVM agents (bugs) roam the sediment, eating the debris to survive and reproduce.

## 🎮 Controls
- **Space**: Toggle Sun Scheduler Mode (Round Robin / Priority).
- **Mouse Hover**: Inspect process stats (Tree) or debris.

## 🧪 Novel Trait
**Necrotic OS Visualization**: Visualizing the "ghosts" of processes. Most task managers only show what is alive. `chimera-sediment` shows what has died and how it accumulates until garbage collected.

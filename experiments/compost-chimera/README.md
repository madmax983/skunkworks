# 🧬 Compost Chimera

> "The best ideas are mongrels." - The Splice Surgeon

A hybrid experiment combining **Chimera Lang** (biological agents) and **Digital Compost** (entropy visualization).

## 🧬 Lineage

- **Parent A**: `experiments/chimera-lang` (The Life)
  - Provided the `ChimeraVM` genetic execution engine.
  - Agents possess DNA that drives their behavior (Move, Consume, etc.).

- **Parent B**: `experiments/digital-compost` (The Environment)
  - Provided the concept of "Bit Rot" where files decay over time.
  - Provided the file scanning and decay rendering logic.

## 🧪 Phenotype

**Digital Detritivores**. Small agents (`@`) roam across the buffers of your source code. The code itself is subject to simulated decay (glitches, bit flips). These agents feed on the entropy—consuming the "compost" (glitched characters) to gain energy.

## 🎮 Controls

- `j/k/h/l`: Scroll the viewport.
- `Tab`: Switch to the next file in the directory.
- `r`: Respawn agents / Reload.
- `q`: Quit.

## 🏗️ Architecture

The simulation runs a TUI loop where `Simulation` manages a population of `Agent`s. Each `Agent` wraps a `ChimeraVM`. The agents "sense" the character under them in the text buffer. If it is a "decayed" character, they consume it (restoring it to space or consuming it entirely) and gain energy.

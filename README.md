# Chimera Prologue ⚛️

> "Order is but a transient island in the ocean of Chaos. We are the storm."

**Chimera Prologue** is a Grid-based Visual Logic Language embedded within the Chimera biological VM. It allows for the construction of "Digital Circuits", "Logic Agents", and "Genetic Machinery" directly on a memory grid.

It unifies the concepts of **Cellular Automata** (Wireworld, Game of Life), **Concatenative Programming** (Forth), and **Biological Simulation** (DNA, Mitosis).

## 🚀 Quick Start

Run the new Evolution example to see the Prologue engine in action:

```bash
cargo run -- --input examples/evolution.pro
```

This will launch the TUI (Text User Interface) showing a live simulation of:
*   A **Logic Grid** with signals propagating through wires.
*   An **Automaton Agent (🤖)** executing a spatial program.
*   **Chaos Runes (K)** injecting entropy.
*   **Custom Runes (£)** triggering DNA execution.

## 📄 The Prologue Format (`.pro`)

Prologue files define the initial state of the simulation.

```prologue
config {
    mode: Orca  # or Normal, Silicon, Life
}

grid {
    "!"  "~"  "~"  "?"
    "."  "."  "."  "."
    "🤖" "~"  "~"  "M"
}

definitions {
    A: {
        strand alpha {
            "Alpha Triggered" print
        }
    }
}

dna {
    strand main {
        "Simulation Started" print
    }
}
```

*   **`config`**: Sets global physics parameters.
*   **`grid`**: Visual layout of the 16x16 memory grid.
*   **`definitions`**: Bind custom characters (Runes) to DNA strands.
*   **`dna`**: The genetic code (ChimeraScript) that runs in the background.

## 🔮 Core Concepts

### 1. The Grid (Petri Dish)
A 16x16 toroidal grid. Each cell can hold:
*   **Runes**: Static logic gates (`&`, `|`, `+`, `~`).
*   **Agents**: Mobile autonomous units (`@`, `🤖`, `K`).
*   **Values**: Integers or Strings.

### 2. Signals
Runes like `!` (Source) emit signals that travel instantly through `~` (Wires). Signals can carry data (Integers, Strings) or simple activation pulses.

### 3. Agents
*   **Seeker (`@`)**: Moves towards signals.
*   **Automaton (`🤖`)**: Executes an internal spatial program string (e.g., `^>v<`).
*   **Chaos (`K`)**: Moves randomly and corrupts neighbors.
*   **Construct (`C`)**: Builds structures based on internal state.

### 4. Runes
*   **Logic**: `&` (AND), `|` (OR), `+` (XOR), `!` (NOT/Source).
*   **Flow**: `^` (Jump), `*` (Split), `#` (Delay).
*   **Memory**: `$` (Scribe), `?` (Sink/Read).
*   **Biology**: `G` (Genesis), `M` (Mutate), `O` (Organelle).

## 🧬 DNA Integration
The Grid and DNA interact:
*   **Grid -> DNA**: A Sink (`?`) receiving a String triggers the DNA strand with that name.
*   **DNA -> Grid**: The `g_write` and `g_read` enzymes allow code to modify the grid.

## 🛠️ Installation

```bash
git clone https://github.com/your-repo/chimera-lang
cd chimera-lang
cargo build --release
```

## 🎮 Controls (TUI)

*   **Arrows**: Move Cursor.
*   **Space**: Step Simulation.
*   **Tab**: Cycle Views (Grid, Genome, Microscope, etc.).
*   **Enter**: Edit Cell / Genome.
*   **C**: Toggle Chaos Mode.
*   **Q**: Quit.

---
*Created by the Mad Scientist "Prologue".*

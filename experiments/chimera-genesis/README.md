# Chimera Genesis 🧬

> "In the beginning, there was the Grid, and the Grid was void. And the Splice Surgeon said, 'Let there be Code', and there was Life."

**Chimera Genesis** is a hybrid experiment combining **Chimera Lang** (Biological VM) with **Cellular Automata** (Klein Life).

It implements a programmable universe where the "laws of physics" are defined by genetic code executed by every atom (cell).

## 🧬 Concept

Unlike traditional Cellular Automata (like Conway's Game of Life) where the rules are hardcoded (B3/S23), in **Chimera Genesis**, every cell contains a **ChimeraVM**.

At each tick:
1. The cell pushes its neighbors' states (0 or 1) onto its VM stack.
2. The cell executes its DNA strand.
3. The result on the stack determines the cell's next state.

This allows for:
- **Evolutionary Physics**: The rules of the universe can mutate.
- **Complex Behaviors**: Cells can use memory, loops, and logic to decide their fate.
- **Hybrid vigor**: The robustness of Life meets the flexibility of Code.

## 🧪 Lineage

- **Parent A**: `experiments/chimera-lang` (The Genetic Engine)
- **Parent B**: `experiments/klein-life` (The Cellular Substrate)
- **Novel Trait**: **Programmable Cellular Physics**. The rule is the code.

## 🕹️ Controls

- **Q**: Quit
- **R**: Reset (Randomize)
- **Space**: Pause/Resume

## 🔬 Implementation Details

- **Grid**: Fixed size TUI grid.
- **VM**: Each cell runs an isolated ChimeraVM instance.
- **DNA**: Currently initializes with a Chimera Assembly implementation of Conway's Game of Life:
    ```rust
    // Logic: Is3 || (Is2 && Self)
    GWrite(Self, 0, 0) // Store self state
    Add x 7            // Sum neighbors
    ... logic ...
    ```

## 📜 License

MIT

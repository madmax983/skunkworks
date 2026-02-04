# Chimera Lang 🧬

*The Evolving Language.*

Chimera Lang is an esoteric programming language designed to simulate biological processes like mutation, recombination, and transcription within a virtual machine. It combines stack-based execution with genetic algorithms and a 2D cellular automata grid.

## Features

*   **Genetic Structure:** Code is organized into DNA (Helix), Strands (Chromosomes), and Genes (Enzymes).
*   **Stack-Based VM:** Standard stack operations for calculation and logic.
*   **Self-Modification:** The code can rewrite itself (`transcribe`), mutate randomly (`mutate`), and recombination (`recombine`) with other strands.
*   **2D Grid Memory (Petri Dish):** A 16x16 grid for spatial storage and cellular automata experiments.
*   **TUI Interface:** A Terminal User Interface to visualize the genome, execution, and the Petri Dish in real-time.

## Enzymes (Instructions)

### Stack Operations
*   `push(n)`: Push a number or string onto the stack.
*   `dup()`: Duplicate the top item.
*   `swap()`: Swap the top two items.
*   `drop()`: Discard the top item.

### Arithmetic
*   `add()`, `sub()`, `mul()`, `div()`: Standard binary operations.

### Control Flow
*   `jump(strand_idx)`: Jump to the start of a specific strand.
*   `brz(strand_idx)`: Branch (jump) if the top of the stack is 0.

### Introspection & Genetics
*   `transcribe()`: Modify a gene's argument at runtime.
    *   Stack args: `value`, `arg_idx`, `gene_idx`, `strand_idx` (top -> bottom).
*   `s_len()`: Push stack length.
*   `helix_len()`: Push number of strands.
*   `gene_len(strand_idx)`: Push number of genes in a strand.

### Petri Dish (Grid)
*   `g_read()`: Read from grid.
    *   Stack args: `y`, `x`. Pushes value at (x, y).
*   `g_write()`: Write to grid.
    *   Stack args: `value`, `y`, `x`.
*   `g_rows()`: Push grid height (16).
*   `g_cols()`: Push grid width (16).

## Controls (TUI)

*   `Space`: Step execution (execute one gene).
*   `M`: Trigger random Mutation.
*   `X`: Trigger Recombination (swap genes between strands).
*   `Q`: Quit.

## Running

```bash
cargo run --release [dna_file]
```
If no file is provided, a default sample DNA is loaded.

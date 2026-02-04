# Chimera Lang 🧬

Chimera is a bio-inspired, stack-based esoteric programming language. It simulates a biological cell with DNA, enzymes, and metabolism.

## Core Concepts

*   **DNA**: The program code, consisting of a Helix of Strands.
*   **Strand**: A sequence of Genes.
*   **Gene**: An instruction (enzyme) with arguments (nucleotides).
*   **Enzymes**: Operations like `push`, `add`, `jump`, `transcribe`.
*   **Metabolism**: Executing instructions consumes **Energy**. If energy reaches 0, the VM halts (Death by Starvation).
*   **Petri Dish**: A 16x16 grid of memory cells for spatial interaction.
*   **Chaos Mode**: A runtime mode where random mutations (radiation) occur automatically.

## Enzymes

### Basic Stack Operations
*   `push(x)`: Push `x` (Int or String) to stack.
*   `dup()`: Duplicate top value.
*   `swap()`: Swap top two values.
*   `drop()`: Discard top value.

### Arithmetic
*   `add()`, `sub()`, `mul()`, `div()`: Standard math operations.

### Control Flow
*   `jump(strand_idx)`: Jump to the start of a strand.
*   `brz(strand_idx)`: Pop value; if 0, jump to strand.

### Biology
*   `photosynthesize()`: Gain 5 Energy.
*   `consume()`: Pop value; gain Energy equal to value (Int) or length (String).
*   `transcribe(strand, gene, arg, value)`: Self-modifying code. Changes a gene's argument at runtime.
*   `s_len()`, `helix_len()`, `gene_len()`: Introspection.

### Epigenetics (Nova Feature)
*   `methylate(strand, gene)`: Silence a gene (skip execution).
*   `demethylate(strand, gene)`: Activate a gene.
*   `recombine(strand_a, strand_b, split)`: Swap tails of two strands.

### Telomeres (Nova Feature)
Strands have a limited lifespan (default: 50 executions). When a strand's telomere count reaches 0, it becomes senescent and is skipped by the VM.
*   `telomerase(amount)`: Extends the current strand's telomere by `amount`. Costs 25 Energy.
*   `t_len()`: Pushes the current strand's telomere length to the stack.

### Petri Dish (New!)
*   `g_read()`: Pop `y`, `x`. Push value at `grid[y][x]`.
*   `g_write()`: Pop `val`, `y`, `x`. Write `val` to `grid[y][x]`.

## Controls (TUI)
*   `Space`: Step execution.
*   `M`: Manually mutate a random gene.
*   `C`: Toggle Chaos Mode (Auto-Mutation).
*   `Q`: Quit.

## Running

```bash
cargo run --release -- --input sample.dna
```

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
*   `print()`: Pop value and print to output log.

### Arithmetic
*   `add()`, `sub()`, `mul()`, `div()`: Standard math operations.

### Control Flow
*   `jump(strand_idx)`: Jump to the start of a strand.
*   `brz(strand_idx)`: Pop value; if 0, jump to strand.

### Biology
*   `photosynthesize()`: Gain 5 Energy.
*   `consume()`: Pop value; gain Energy equal to value (Int) or length (String).
*   `transcribe(strand, gene, arg, value)`: Self-modifying code. Changes a gene's argument at runtime.
*   `genome()`: Introspection. Pushes current strand length, then each gene operator as a string.
*   `s_len()`, `helix_len()`, `gene_len()`: Introspection.

### Epigenetics (Nova Feature)
*   `methylate(strand, gene)`: Silence a gene (skip execution).
*   `demethylate(strand, gene)`: Activate a gene.
*   `recombine(strand_a, strand_b, split)`: Swap tails of two strands.

### Cell Cycle (Nova Feature)
*   `mitosis(strand_idx)`: Clones the target strand and appends it to the Helix. Inherits epigenetic state. Costs 30 Energy.
*   `apoptosis(strand_idx)`: Clears all genes in the target strand, effectively killing it. Removes epigenetic state. Costs 10 Energy.
*   `s_index()`: Pushes the current strand index to the stack. Useful for self-reference in `mitosis`.

### Telomeres (Nova Feature)
Strands have a limited lifespan (default: 50 executions). When a strand's telomere count reaches 0, it becomes senescent and is skipped by the VM.
*   `telomerase(amount)`: Extends the current strand's telomere by `amount`. Costs 25 Energy.
*   `t_len()`: Pushes the current strand's telomere length to the stack.

### Quantum Entanglement (Nova Feature)
*   `entangle(strand_a, strand_b)`: Creates a quantum link between two strands. Any mutation (random or via `transcribe`) to one strand is instantly propagated to the other.
*   `decohere(strand_idx)`: Breaks the quantum link for the target strand and its partner.

### Organelles (Nova Feature)
*   `spawn(type, strand_idx)`: Spawns an active agent (Organelle) at the current grid location. The Organelle executes the code in `strand_idx` independently.
    *   **Types**:
        *   `0`: **Worker** (Standard execution).
        *   `1`: **Chloroplast** (Photosynthesis). Gains energy from `light_grid` (passive).
        *   `2`: **Mitochondria** (Powerhouse). Reduces metabolic cost.
        *   `3`: **Lysosome** (Recycler). Consumes `waste_grid` to produce energy.

### Petri Dish (New!)
*   `g_read()`: Pop `y`, `x`. Push value at `grid[y][x]`.
*   `g_write()`: Pop `val`, `y`, `x`. Write `val` to `grid[y][x]`.
*   `radiate()`: Arguments: `val`, `r`, `y`, `x` (top). Write `val` to all cells within radius `r` of `(y, x)`.
*   `siphon()`: Arguments: `r`, `y`, `x` (top). Sum values of all cells within radius `r` of `(y, x)`, clear them, and push sum.

### Virology (Grid Execution)
*   `virus()`: Pop `y`, `x`. Executes the value at `grid[y][x]` as an enzyme.
    *   If `Int(n)`, it behaves like `push(n)`.
    *   If `Str(s)`, it executes the enzyme named `s`.
*   `incubate()`: (Nova Feature) Arguments: `len`, `y`, `x` (top). Reads `len` cells horizontally from grid starting at `(y, x)` and creates a new Strand (Horizontal Gene Transfer).
    *   `Int(n)` becomes `push(n)`.
    *   `Str(s)` becomes `s()`.

### Advanced Control Flow
*   `jump_s()`: Pop target index from stack and jump to that strand.
*   `brz_s()`: Pop target index and condition. If condition is 0, jump to target strand.

### Experimental Nova Enzymes (Undocumented)
The following enzymes are available in the Nova feature set but are currently experimental:
`Rift`, `Seal`, `Shape`, `Simulate`, `Dream`, `Chemotaxis`, `Identity`, `Differentiate`, `Sporulate`, `Germinate`, `CrisprScan`, `Cas9Cut`, `Ligase`, `Integrase`, `Excision`, `Secrete`, `Detect`, `Absorb`, `Migrate`, `Detox`, `WRead`, `Call`, `Ret`, `Bind`, `Unbind`, `Conjugate`, `Gravitate`, `Lumine`, `SenseLight`, `Broadcast`, `Tune`, `Membrane`, `Osmosis`, `Symbiosis`, `Lysis`, `Reflex`, `Compile`, `Decompile`, `Sonar`.

## Controls (TUI)
*   `Space`: Step execution.
*   `M`: Manually mutate a random gene.
*   `C`: Toggle Chaos Mode (Auto-Mutation).
*   `Q`: Quit.

## ChimeraScript 🧪

ChimeraScript is a high-level, Concatenative syntax for writing Chimera DNA. It compiles directly to the underlying DNA structure.

### Features
*   **Clean Syntax**: No brackets, minimal punctuation.
*   **Named Strands**: Use `strand name { ... }` instead of array indices.
*   **Labels**: Jump to strand names directly (e.g., `jump(loop)`).
*   **Literals**: `5` compiles to `push(5)`, `"text"` to `push("text")`.
*   **Comments**: Use `#` for comments.

### Example

```chimera
strand main {
    "Hello World" print
    5 3 add print

    # Conditional jump
    0 eq brz(end)

    jump(loop)
}

strand loop {
    # ...
}

strand end {
    apoptosis
}
```

## Running

You can run legacy DNA files (`.dna`) or new ChimeraScript files (`.chs`).

```bash
cargo run --release -- --input sample.dna
# OR
cargo run --release -- --input examples/genesis.chs
```

To run in headless mode (no TUI), use the `--headless` flag:
```bash
cargo run --release -- --input sample.dna --headless
```

## Nova Features

Some features (Epigenetics, Cell Cycle, Telomeres, `incubate`) are part of the "Nova" expansion and are gated behind a feature flag. To use them, you must enable the `nova` feature:

```bash
cargo run --release --features nova -- --input sample.dna
```

## Library Usage

Chimera can be used as a Rust library to embed the VM in other applications.

Add to your `Cargo.toml`:
```toml
[dependencies]
# Note: Adjust path to point to the chimera-lang directory relative to your project
chimera-lang = { path = "../chimera-lang", features = ["nova"] }
```

Example `main.rs`:
```rust
use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::ChimeraVM;

fn main() {
    let dna = Dna { helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);
    // ... configure VM ...
    vm.step();
}
```

See `examples/story_demo.rs` for a full example of programmatic usage (Requires `nova` feature).

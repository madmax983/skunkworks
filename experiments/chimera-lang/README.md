# Chimera Lang 🧬

> ℹ️ **NOTE**: Advanced features (including `incubate` and `Pandemonium`) are enabled by default via the `nova` feature flag.

Chimera is a bio-inspired, stack-based esoteric programming language. It simulates a biological cell with DNA, enzymes, and metabolism.

## 🗺️ Documentation Map

*   **[PROLOGUE.md](PROLOGUE.md)**: The manual for the **Rune Logic** system (Digital Circuits on the Grid).
*   **[GRIMOIRE.md](GRIMOIRE.md)**: Advanced documentation for **Nova** features (Time Travel, Quantum, Metaphysics).
*   **[src/lib.rs](src/lib.rs)**: The architectural overview of the Rust codebase (The "Book of Chimera").

## 🚀 Quick Start

Run the Story Demo to see the engine in action. This demo initializes a VM, compiles a sample ChimeraScript, and runs a TUI (Text User Interface) simulation where you can observe the organism's lifecycle.

> **Note**: This demo requires the `nova` feature (enabled by default).

```bash
cargo run -p chimera-lang --example story_demo
```

**Expected Output:**
You will see a TUI with multiple panes:
- **Genome**: The executing code.
- **Grid**: The 16x16 Petri dish where cells move and interact.
- **Microscope**: Details of the currently selected cell.
- **Log**: Output from `print()` enzymes.

Press `Space` to step the simulation, or `C` to toggle Chaos Mode.

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
*   `reincarnate(strand_idx)`: Kills the target strand and immediately spawns a mutated copy of it at the end of the helix. Costs 50 Energy.
*   `s_index()`: Pushes the current strand index to the stack. Useful for self-reference in `mitosis`.

### Telomeres (Nova Feature)
Strands have a limited lifespan (default: 50 executions). When a strand's telomere count reaches 0, it becomes senescent and is skipped by the VM.
*   `telomerase(amount)`: Extends the current strand's telomere by `amount`. Costs 25 Energy.
*   `t_len()`: Pushes the current strand's telomere length to the stack.

### Quantum Entanglement (Nova Feature)
*   `entangle(strand_a, strand_b)`: Creates a quantum link between two strands. Any mutation (random or via `transcribe`) to one strand is instantly propagated to the other.
*   `decohere(strand_idx)`: Breaks the quantum link for the target strand and its partner.

### Quantum Cybernetics (Nova Feature)
Links the Holographic genetics with the physical Grid.
*   `quantum_scribe(threshold)`: Collapses the local Hologram wavefunction into a physical grid character. If magnitude > threshold, phase determines the character.
*   `quantum_scan(weight)`: Encodes the local physical grid character into the Hologram.

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

### Metaphysics (Nova)
*   `rift(y2, x2, y1, x1)`: Opens a spatial portal from `(x1, y1)` to `(x2, y2)`.
*   `seal(y, x)`: Closes a portal at `(x, y)`.
*   `shape(topology_id)`: Warps grid topology. 0=Plane, 1=Torus, 2=CylinderH, 3=CylinderV, 4=Klein, 5=Mobius, 6=Hyperbolic. Cost: 100.
*   `phase_shift(phase_id)`: Changes matter state. 0=Corporeal, 1=Ethereal (pass walls), 2=Crystalline (immobile), 3=Flux (fast). Cost: 50.
*   `isomerize()`: Toggles chirality (Left <-> Right). Inverts math and logic directions.
*   `spirit()`: Pauses execution and waits for external user input.

### Simulation (Nova)
*   `simulate(ticks, strand_idx)`: Runs a sandboxed simulation of `strand_idx` for `ticks`. Pushes `[result, energy, status]`. Cost: 50 + ticks.
*   `dream(ticks, strand_idx)`: Simulates a mutated version of `strand_idx`. If energy outcome is positive, adopts the mutation. Cost: 50 + ticks/2.
*   `prophecy(ticks)`: Predicts if current execution leads to death within `ticks`. Pushes 1 (Death) or 0 (Life).

### Physics (Nova)
*   `gravitate(radius)`: Pulls all objects within `radius` towards the center. Cost: Variable.
*   `lumine(intensity, radius)`: Emits light. Chloroplasts harvest energy from this.
*   `sense_light()`: Pushes local light level.
*   `osmosis(dy, dx)`: Moves through membranes/walls. High energy cost.
*   `membrane(mask)`: Toggles wall boundaries (1=N, 2=S, 4=E, 8=W).
*   `broadcast(channel, value)`: Sends value to global ether channel.
*   `tune(channel)`: Receives value from global ether channel.
*   `signal(channel, value)`: IPC Send (External).
*   `receive(channel)`: IPC Receive (External).

### Chemistry (Nova)
*   `alchemy()`: Transmutes grid cells based on neighbors (e.g., Fire + Water = Steam). Cost: 5.
*   `secrete(channel, amount)`: Releases hormones at current location.
*   `detect(channel)`: Reads hormone level.
*   `absorb(channel, amount)`: Consumes hormones.
*   `detox(radius)`: Clears waste within radius.
*   `w_read()`: Reads local waste level.

### Microbiology (Nova)
*   `spawn(type, strand_idx)`: Spawns an organelle. Types: 1=Chloroplast, 2=Mitochondria, 3=Lysosome, 4=Ribosome, 6=Alchemist.
*   `identity()`: Pushes current organelle type ID.
*   `differentiate(type)`: Changes organelle type. Cost: 50.
*   `symbiosis(dy, dx)`: Merges with an organelle at target, absorbing its stack and IP.
*   `lysis()`: Ejects the last absorbed symbiote as a new organelle.
*   `chemotaxis(channel)`: Pushes `dy, dx` towards highest hormone concentration.
*   `hyphae()`: Spawns a fungal network node.
*   `connect(y, x)`: Links current hyphae to another.
*   `transport(val, y, x)`: Sends value instantly across mycelium.
*   `spore_cloud(radius, density)`: Randomly spawns hyphae nearby.

### Genetics II (Nova)
*   `splice(strand_a, strand_b, method)`: Combines two strands. 0=Interleave, 1=Crossover, 2=Merge.
*   `recombine(strand_a, strand_b, split)`: Swaps tails of two strands.
*   `crispr_scan(target, guide)`: Scans target strand for pattern matching guide. Returns index.
*   `cas9_cut(strand, index)`: Cuts strand at index. Tail becomes new strand.
*   `ligase(recipient, donor)`: Appends donor strand to recipient.
*   `integrase(strand, gene_idx, name, arg)`: Inserts a new gene.
*   `excision(strand, gene_idx)`: Removes a gene.
*   `conjugate(strand, y, x, dir)`: Writes DNA sequence onto the grid.
*   `incubate(len, y, x)`: Reads grid sequence into new DNA.
*   `compile(string)`: Compiles string to DNA.
*   `decompile(strand)`: Decompiles DNA to string.

### Necromancy (Nova)
*   `bury(strand)`: Moves strand to graveyard.
*   `exhume()`: Restores last buried strand.
*   `seance()`: Executes last buried strand as a ghost (ephemeral).
*   `mourn()`: Gains energy from graveyard size.
*   `reincarnate(strand)`: Kills strand, spawns mutated copy.

### Functional (Nova)
*   `eval(string)`: Executes string as code.
*   `map(junction, function)`: Applies function (strand/string) to each item.
*   `fold(junction, init, function)`: Reduces junction.
*   `filter(junction, predicate)`: Filters items.
*   `zip(j1, j2)`: Combines two junctions.
*   `match(pattern, target)`: Glob matching.
*   `supernova()`: Explodes current strand, scattering genes on grid.
*   `singularity()`: Merges all strands into one.

### Raku Features (Nova)
*   `>>+<<`, `>>-<<`, `>>*<<`, `>>/<<`: Hyper-operators for element-wise arithmetic on lists.
*   `[+]`, `[*]`, etc.: Reduction (Fold) operators.
*   `X+`, `X*`, etc.: Cross-product with operator.
*   `Z+`, `Z*`, etc.: Zip with operator.

### Babel-19 (Semantic Grid)
The Babel system now allows for "Live Parsing" of the grid as executable code.
*   **Semantic Actions**: Embed ChimeraScript in grid cells using `{ code }`.
    *   Example: `"Hello" -> { "Found it" print } -> !`
*   `babel_live(y, x, input)`: Triggers the parser at (y,x) with the given input string.

### Prologue (Rune Logic)
A new Rune-based Logic Language that runs on the grid.
*   Toggle with `[` or `OpCode::Prologue`.
*   Runes: `?` (Query), `!` (Fact), `@` (Agent).
*   See `PROLOGUE.md` for details.

## Controls (TUI)
*   `Space`: Step execution.
*   `Tab`: Cycle Views (Genome -> Grid -> Microscope -> Cortex -> Metaphysics -> Topology -> Laboratory).
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

> **Note**: For non-interactive environments (CI, scripts), append the `--headless` flag.

```bash
# Basic usage with standard DNA files (from repository root)
cargo run -p chimera-lang --release -- --input experiments/chimera-lang/sample.dna

# Running complex examples like Genesis
cargo run -p chimera-lang --release -- --input experiments/chimera-lang/examples/genesis.chs

# Headless mode (no TUI)
cargo run -p chimera-lang --release -- --input experiments/chimera-lang/examples/genesis.chs -- --headless
```

## Nova Features

Some features (Epigenetics, Cell Cycle, Telomeres, `incubate`) are part of the "Nova" expansion. This feature is enabled by default.

## Library Usage

Chimera can be used as a Rust library to embed the VM in other applications.

Add to your `Cargo.toml`:
```toml
[dependencies]
# Note: Adjust path to point to the chimera-lang directory relative to your project
chimera-lang = { path = "../chimera-lang" }
```

Example `main.rs`:
```rust
use chimera_lang::prelude::*;

fn main() {
    // Create a simple organism that prints "Hello"
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("Hello".to_string())] },
        Gene { op: OpCode::Print, args: vec![] },
        // Note: For integers, use Nucleotide::Number(n)
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(42)] },
        Gene { op: OpCode::Print, args: vec![] },
    ];
    let dna = Dna {
        helix: Helix { strands: vec![Strand { genes }] },
        evolution_config: None,
    };
    let mut vm = ChimeraVM::new(dna);

    vm.step(); // Push "Hello"
    vm.step(); // Print "Hello"
    vm.step(); // Push 42
    vm.step(); // Print 42

    // The VM captures output in `vm.output` (Vec<String>) instead of printing to stdout.
    for line in &vm.output {
        println!("{}", line);
    }
}
```

### Running ChimeraScript from Rust

You can also parse and run ChimeraScript code directly using the compiler:

```rust
use chimera_lang::prelude::*;
use chimera_lang::compiler;

fn main() {
    let source = r#"
        strand main {
            "Hello from Script" print
            42 print
        }
    "#;

    // Compile the source string into DNA
    // The second argument is an optional path for imports (None here)
    let dna = compiler::compile(source, None).expect("Failed to compile");

    let mut vm = ChimeraVM::new(dna);

    // Run until halted or for a max number of steps
    for _ in 0..100 {
        if vm.halted { break; }
        vm.step();
    }

    for line in &vm.output {
        println!("{}", line);
    }
}
```

See `examples/story_demo.rs` for a full example of programmatic usage:

> **REQUIRES FEATURE NOVA**

```bash
cargo run -p chimera-lang --example story_demo
```

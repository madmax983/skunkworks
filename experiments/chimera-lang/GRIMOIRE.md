# The Grimoire of Prologue ⚛️

> "To define is to destroy. To describe is to decay. But to code... to code is to Become." - The Mad Scientist

Welcome to the **Grimoire**, the forbidden manual of `chimera-lang`. Here lies the knowledge to weave DNA, summon organelles, and warp the fabric of the simulation.

## 🧬 Core Concepts

Chimera is a bio-inspired, stack-based esoteric programming language. It simulates a biological cell with DNA, enzymes, and metabolism.

*   **DNA**: The program code, consisting of a Helix of Strands.
*   **Strand**: A sequence of Genes.
*   **Gene**: An instruction (enzyme) with arguments (nucleotides).
*   **Enzymes**: Operations like `push`, `add`, `jump`, `transcribe`.
*   **Metabolism**: Executing instructions consumes **Energy**. If energy reaches 0, the VM halts (Death by Starvation).
*   **Petri Dish**: A 16x16 grid of memory cells for spatial interaction.

## 🧪 ChimeraScript

ChimeraScript (`.chs`) is the high-level language used to write DNA.

### Basic Syntax

```chimera
strand main {
    "Hello World" print
    5 3 add print
}
```

### Organelles (New!)

You can define specialized agents called **Organelles** with their own DNA and traits.

```chimera
organelle Walker {
    # Genes executed by the organelle
    move(1, 0)
    photosynthesize
    jump(0)
}

strand main {
    # Spawn a Walker
    SpawnWalker
}
```

This compiles to a DNA strand `Walker_DNA` and a macro `SpawnWalker` that invokes the `spawn` enzyme.

## 🔮 Nova Features

The **Nova** expansion adds metaphysical and quantum capabilities.

### Time Travel (Chronos)
*   `time_warp(factor, radius)`: Dilate time in a local area.
*   `time_loop(id)`: Create a closed timelike curve.
*   `retrograde(ticks)`: Reverse entropy locally.

### Quantum Mechanics
*   `superpose(val1, prob1, val2, prob2)`: Create a quantum superposition.
*   `collapse()`: Observe the stack, collapsing wavefunctions.
*   `entangle(strand_a, strand_b)`: Link two strands quantumly.

### The Void
*   `void_rift(y, x)`: Tear a hole in the grid.
*   `entropy_surge()`: Increase global chaos.

## 🧠 Oracle (Logic Engine)

The Oracle allows the organism to reason about itself and the world using Prolog-like logic.

*   `prolog_call(query)`: Executes a logic query and pushes bindings to the stack.
    *   Example: `prolog_call(metabolism(?E))` -> pushes `[[["?E", 50]]]`
*   `manifest(query, transform)`: Applies a transformation to all states matching the query.
    *   Example: `manifest(energy(?E), spawn_strand([["push", ?E]]))`

### Dynamic Predicates
*   `metabolism(?Energy)`: Unifies with current energy level.
*   `cell(?X, ?Y, ?Val)`: Unifies with grid cells.
*   `gene(?Strand, ?Idx, ?Op)`: Unifies with genome.

## 🕹️ Interface (Prologue)

The TUI is your laboratory.

*   **Tab**: Cycle Views (Genome, Grid, Microscope, **Grimoire**, etc.)
*   **Space**: Step execution.
*   **Q**: Quit.

### The Grimoire View
You are reading it. Use **Up/Down** arrows to scroll through this ancient text.
The panel on the right shows the **Knowledge Base** facts and query results.

---
*End of Fragment.*

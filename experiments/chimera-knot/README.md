# Chimera Knot 🪢🧬

**Lineage:** `knot-archiver` × `chimera-lang`

A hybrid experiment where the program code is a physical Quipu knot structure.
The organism's genome is not a linear strand, but a collection of knotted cords.

## Concept

- **Genetic Material**: Code is stored in `Cord`s.
- **Instructions**: `Knot`s on the cord.
- **Execution**: The "bead" (Program Counter) slides down the cord.
- **Control Flow**: Jumping between cords changes the "thread" of execution. Recursion is visualized as diving into subsidiary cords (implemented as function calls).

## Usage

```bash
cargo run -p chimera-knot
```

Controls:
- `Space`: Step execution.
- `r`: Fast-forward.
- `q`: Quit.

## Parents

- **knot-archiver**: Provided the data structure (Quipu, Cord, Knot) and the TUI visualization style.
- **chimera-lang**: Provided the VM architecture and OpCode definitions.

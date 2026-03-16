# 🧬 Quipu Chimera

**Lineage:** `quipu-symphony` × `chimera-lang`

A visualization where ChimeraVM execution is mapped to an Incan Quipu. The DNA strand is a vertical cord. Opcodes are knots. The Program Counter (PC) is a gravity-driven playhead. Execution is sonified.

## Concept

- **Code as Knots:** Each instruction (Gene) in the DNA is represented as a knot on a vertical cord.
- **Gravity Execution:** The execution flow follows gravity (downwards). Jumps fight against or with gravity.
- **Sonic Debugging:** As the playhead (PC) hits a knot, it triggers a sound corresponding to the instruction type.

## Controls

- `Space`: Pause/Resume execution.
- `Up/Down`: Adjust execution speed (Gravity).
- `R`: Reset the VM.
- `Q`: Quit.

## Technical Details

- **VM:** Uses `chimera-lang` (Nova/Silicon features supported if enabled).
- **TUI:** Rendered using `ratatui`.
- **Audio:** Synthesized using `cpal` (optional feature).

## Running

```bash
cargo run -p quipu-chimera --features audio
```

To run without audio (e.g. in headless environments):
```bash
cargo run -p quipu-chimera
```

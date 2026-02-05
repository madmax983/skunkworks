# Trace Tardis 🧬

> "It's bigger on the inside... because it's recursive."

**Trace Tardis** is a hybrid experiment born from the recombination of `trace-fold` and `alloc-tardis`. It visualizes a Rust stack trace as an infinite sequence of recursive rooms.

## 🧬 Lineage

- **Parent A:** `experiments/trace-fold`
  - *Allele:* Stack Trace Parsing logic.
  - *Trait:* Transforms raw text data into structured segments.
- **Parent B:** `experiments/alloc-tardis`
  - *Allele:* Recursive Room Rendering (Macroquad + glScissor).
  - *Trait:* The "infinite zoom" capability where rooms contain portals to other rooms.

## 🧪 Phenotype

The experiment takes a hardcoded raw stack trace (simulated) and generates a `World` where:
- Each **Stack Frame** is a **Room**.
- The function call that leads to the next frame is a **Portal** inside that room.
- Navigating "deeper" into the stack means flying through these portals.
- The recursive rendering engine allows you to see deep into the future of the stack execution.

## 🎮 Controls

- **WASD / Arrows**: Pan the camera.
- **+/-** or **Q/E**: Zoom In/Out.
- Fly into a portal (Red/Blue box) to transition to the next stack frame.

## ⚠️ Notes

- This is a visualization experiment using `macroquad`.
- The stack trace is currently hardcoded in `main.rs` for demonstration.

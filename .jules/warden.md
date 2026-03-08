**2025-01-20 - [Clamp Scissor Dimensions]**
**Threat:** The `intersect_rect` function in `experiments/chimera-tardis/src/safe_gl.rs` allowed negative width or height values to be passed to saturating addition, potentially resulting in underflow/overflow or negative dimensions being passed to `glScissor`, which could invoke Undefined Behavior depending on driver implementation.
**Defense:** Explicitly clamped `w` and `h` inputs via `.max(0)` before calculating the intersection bounds to ensure mathematical soundness and prevent negative `glScissor` arguments.

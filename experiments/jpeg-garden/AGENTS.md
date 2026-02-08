# AGENTS.md

## Scope
This file applies to `experiments/jpeg-garden`.

## Guidelines
-   **Preservation**: Document every change. This is a study in decay, not a product.
-   **Style**: Use `rustfmt` and `clippy`.
-   **Audio**: Keep volume levels reasonable (clamp output).

## Protocols
-   If you modify the DCT/IDCT, ensure it remains reversible (within float precision).
-   If you add new decay modes, name them metaphorically (e.g. "Moss", "Rust", "Mold").

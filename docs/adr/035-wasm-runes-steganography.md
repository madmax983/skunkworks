# 035. WASM Runes Steganography & Execution

## Status
Accepted

## Context
We need a way to distribute executable logic in a format that feels "magical" and visually integrated with the creative coding aesthetic of the Skunkworks repository. The goal is to embed WASM binaries into procedurally generated "Rune" images, allowing for steganographic distribution of sandboxed code.

Standard steganography tools often use linear LSB encoding or spread data randomly. However, for "Runes," the visual core of the image is the most important part, and we want the data to be physically located near the center of the glyph to reinforce the metaphor of "power radiating from the core."

Furthermore, executing this embedded code requires a WASM runtime. The WASM ecosystem is rapidly evolving, with the Component Model (Preview 2) introducing significant complexity and breaking changes for simple WASI use cases.

## Decision
We decided to implement a custom steganography and execution pipeline in `experiments/wasm-runes`:

1.  **Custom Spiral LSB Encoding:**
    *   We implement a `SpiralIter` that iterates coordinates starting from the center of the image and spiraling outwards.
    *   The payload (WASM binary) is embedded into the Least Significant Bits (LSB) of the image pixels following this spiral path.
    *   This ensures that the critical data is concentrated in the visual center, allowing the edges of the image to be less "noisy" or even cropped/damaged without immediate data loss (though current implementation does not support error correction).

2.  **Pinned WASM Runtime:**
    *   We use `wasmtime` version **14.0** and `wasmtime-wasi` version **14.0**.
    *   We explicitly chose *not* to upgrade to v18.0+ or the Component Model at this time.
    *   This allows us to use the simpler, synchronous `wasi-common` API for file system and stdio access, which is sufficient for our single-module "Rune" scripts.

3.  **Procedural Generation:**
    *   The visual appearance of the Rune is deterministically generated from the hash of the payload.
    *   This means the "look" of the rune is a unique fingerprint of the code it contains.

## Consequences

### Positive
*   **Aesthetic Integration:** The spiral encoding aligns the data distribution with the visual composition (radial glyphs).
*   **Simplicity:** Sticking to `wasmtime` 14.0 avoids the boilerplate and complexity of the Wasm Component Model, making the `vm.rs` implementation concise.
*   **Portability:** Runes are self-contained image files that can be shared and executed by anyone with the `wasm-runes` tool.

### Negative
*   **Tech Debt:** Pinning `wasmtime` to an older version means we miss out on performance improvements and security patches in newer versions. Upgrading later will require a significant rewrite of the host-guest interface.
*   **Fragility:** LSB steganography is fragile. Resizing, compressing (JPEG), or color-correcting the image will destroy the payload. Runes must be stored as PNG.

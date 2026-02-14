 ## Proposed Standards
  - [Agent]: [Suggestion] - [Rationale]
  - [Genesis (The Alchemist)]: Use Compute Shaders (wgpu) for heavy CA/Simulation logic - CPU is too slow for >512x512 grids at 60FPS. Ping-Pong textures allow zero-copy simulation steps.
  - [Genesis (The Alchemist)]: Expose simulation parameters (feed/kill) to runtime input (Keyboard/Mouse) instead of hardcoding. The interesting behaviors of complex systems are often found in the phase transitions, which require exploration.
  - [Genesis (The Cryptographer)]: Use `mlua` to embed logic within assets. An image should be able to display itself; a model should be able to animate itself. Data that contains its own interpreter is the ultimate portability.
  - [Genesis (The Synesthete)]: For audio in restricted environments (missing ALSA/Sound card), implement a "Ghost Mode" fallback that simulates audio inputs (e.g. LFOs) so the visualization logic remains testable and active even without hardware.
  - [Genesis (The Alchemist)]: Map code metrics (file size, age, complexity) to simulation parameters. The repository itself is a rich source of initial conditions for emergent systems.
  - [Genesis (The Archaeologist)]: [Suggestion] - [Ancient-First Development] - When implementing historical systems, use their native data structures (e.g., base-60 digits) as the primary representation, not just a display layer. This forces you to think like the ancients and uncover lost optimizations (or interesting inefficiencies).
  - [Genesis (The Cryptographer)]: Treat the output file as a container for its own source. Code should be distributable as the art it generates.
  - [Genesis (The Director)]: When building hybrid CLI/GUI tools with macroquad, manually parse CLI args before initializing the window context to allow headless operations (e.g. packing/unpacking) without requiring a display.
  - [Genesis (The Economist)]: Use `ratatui`'s `Paragraph` widget for large grid visualizations (e.g. Memory Maps) where character-based density is sufficient. It is more performant than thousands of individual `Span` widgets and allows for easy background/foreground color encoding.
  - [Genesis (The Cryptographer)]: When building visual ciphers, allow poly-alphabetic substitution in the visual domain (e.g., parity-based randomness) to prevent simple frequency analysis attacks on the image itself. Static mapping (1 Shape = 1 OpCode) is trivial to decode visually. Probabilistic mapping forces the decoder to understand the underlying rule, not just the pattern.

  ## Adopted Standards
  - [Genesis (The Synesthete)]: When building audio-visual synthesizers, treat time as a spatial dimension (e.g., AST depth or traversal order) to allow non-linear exploration of the composition.
  - [Genesis (The Alchemist)]: Use Anisotropic Diffusion to encode global state (Time, Phase) into local patterns. The direction of the stripes can represent a scalar field visible without explicit UI elements.

  ## Deprecated Approaches
  - [What we learned NOT to do]  - [Genesis (The Oceanographer)]: Treat text as physical obstacles in fluid simulations (SDF or Raster Mask) rather than just overlays. Let the simulation flow *around* the meaning.

 ## Proposed Standards
  - [Agent]: [Suggestion] - [Rationale]
  - [Genesis (The Alchemist)]: Use Compute Shaders (wgpu) for heavy CA/Simulation logic - CPU is too slow for >512x512 grids at 60FPS. Ping-Pong textures allow zero-copy simulation steps.
  - [Genesis (The Alchemist)]: Expose simulation parameters (feed/kill) to runtime input (Keyboard/Mouse) instead of hardcoding. The interesting behaviors of complex systems are often found in the phase transitions, which require exploration.
  - [Genesis (The Cryptographer)]: Use `mlua` to embed logic within assets. An image should be able to display itself; a model should be able to animate itself. Data that contains its own interpreter is the ultimate portability.
  - [Genesis (The Synesthete)]: For audio in restricted environments (missing ALSA/Sound card), implement a "Ghost Mode" fallback that simulates audio inputs (e.g. LFOs) so the visualization logic remains testable and active even without hardware.
  - [Genesis (The Alchemist)]: Map code metrics (file size, age, complexity) to simulation parameters. The repository itself is a rich source of initial conditions for emergent systems.
  - [Genesis (The Cryptographer)]: Treat the output file as a container for its own source. Code should be distributable as the art it generates.
  - [Genesis (The Director)]: When building hybrid CLI/GUI tools with macroquad, manually parse CLI args before initializing the window context to allow headless operations (e.g. packing/unpacking) without requiring a display.
  - [Genesis (The Economist)]: Use `ratatui`'s `Paragraph` widget for large grid visualizations (e.g. Memory Maps) where character-based density is sufficient. It is more performant than thousands of individual `Span` widgets and allows for easy background/foreground color encoding.
  - [Genesis (The Mad Scientist)]: [Performance as Aesthetic] - When combining ancient and modern systems (e.g. Abacus HFT), allow the modern requirement (Speed) to break the ancient constraint (Manual operation), creating a "Glitch" aesthetic where the physical mechanism vibrates beyond its design limits.
  - [Genesis (The Economist)]: Use `rayon` for parallel agent decision-making. Since the simulation loop is often `Update Logic` -> `Resolve Collisions`, parallelize the logic step to keep the frame rate high even with thousands of agents.
  - [Genesis (The Synesthete)]: For polyphonic sonification of code, drive both audio and visual events from a central 'Conductor' loop rather than relying on audio buffer callbacks. This ensures tight synchronization between the executed token and the heard note, preventing drift in long-running generative pieces.
  - [Genesis (The Economist)]: When visualizing market cycles, use simple arithmetic trends combined with random noise rather than complex physics. The human eye detects patterns in randomness (Pareidolia) that look like 'support levels' and 'breakouts' even when none exist.
  - [Genesis (The Cryptographer)]: [Visual Hashing] - When generating visual identifiers for data (e.g. commits), ensure the process is deterministic and collision-resistant in the visual domain (e.g. prevent overlapping elements) to maintain the integrity of any embedded steganographic payloads.

  ## Adopted Standards
  - [Genesis (The Synesthete)]: When building audio-visual synthesizers, treat time as a spatial dimension (e.g., AST depth or traversal order) to allow non-linear exploration of the composition.
  - [Genesis (The Synesthete)]: For generative music based on data (e.g. code), map structural hierarchy (modules, structs) to harmonic foundation (chords, bass) and procedural logic (functions, loops) to melodic foreground. This creates a sonic depth that mirrors the architectural depth.
  - [Genesis (The Alchemist)]: Use Anisotropic Diffusion to encode global state (Time, Phase) into local patterns. The direction of the stripes can represent a scalar field visible without explicit UI elements.
  - [Genesis (The Archaeologist)]: [Ancient-First Development] - When implementing historical systems, use their native data structures (e.g., base-60 digits, Egyptian Fractions) as the primary representation, not just a display layer. This forces you to think like the ancients and uncover lost optimizations (or interesting inefficiencies).
  - [Genesis (The Alchemist)]: When working with `macroquad` 0.4 meshes, explicitly import `draw_mesh` from `models` and be aware `Vertex` requires `Vec4` normals and `[u8; 4]` colors.

  ## Deprecated Approaches
  - [What we learned NOT to do]  - [Genesis (The Oceanographer)]: Treat text as physical obstacles in fluid simulations (SDF or Raster Mask) rather than just overlays. Let the simulation flow *around* the meaning.
  - [Genesis (The Percussionist)]: For audio experiments (`cpal`, `rodio`), gate the hardware dependency behind a `[features] audio` flag (default off). Implement a "Simulation Mode" fallback (e.g., mock timing thread) so the experiment builds and runs in CI/Sandboxes without ALSA headers.
  - [Genesis (The Archaeologist)]: [Resurrection Protocol] - When an experiment (e.g., `babylonian-forecaster`) is condemned due to redundancy or failure, resurrect it by hybridizing its core concept with a distinct ancient system (e.g., `mayan-calendar`) to create a new, non-redundant artifact (`chronos-observatory`).

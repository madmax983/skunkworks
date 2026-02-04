 ## Proposed Standards
  - [Agent]: [Suggestion] - [Rationale]
  - [Genesis (The Alchemist)]: Use Compute Shaders (wgpu) for heavy CA/Simulation logic - CPU is too slow for >512x512 grids at 60FPS. Ping-Pong textures allow zero-copy simulation steps.
  - [Genesis (The Synesthete)]: For audio in restricted environments (missing ALSA/Sound card), fallback to generating WAV files via `hound` and purely visual TUI feedback. Real-time synthesis via `cpal`/`rodio` is brittle in sandboxes.
  - [Genesis: The Economist]: Separating 'Matching' logic from 'Settlement' logic in market simulations allows for complex asset transfers (like MemoryBlocks) without cluttering the core pricing engine.

  ## Adopted Standards
  - [Consensus items agents can reference]

  ## Deprecated Approaches
  - [What we learned NOT to do]
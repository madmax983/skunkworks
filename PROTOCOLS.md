 ## Proposed Standards
  - [Agent]: [Suggestion] - [Rationale]
  - [Genesis (The Alchemist)]: Use Compute Shaders (wgpu) for heavy CA/Simulation logic - CPU is too slow for >512x512 grids at 60FPS. Ping-Pong textures allow zero-copy simulation steps.
  - [Genesis (The Alchemist)]: Expose simulation parameters (feed/kill) to runtime input (Keyboard/Mouse) instead of hardcoding. The interesting behaviors of complex systems are often found in the phase transitions, which require exploration.
  - [Genesis (The Synesthete)]: For audio in restricted environments (missing ALSA/Sound card), implement a "Ghost Mode" fallback that simulates audio inputs (e.g. LFOs) so the visualization logic remains testable and active even without hardware.

  ## Adopted Standards
  - [Consensus items agents can reference]

  ## Deprecated Approaches
  - [What we learned NOT to do]
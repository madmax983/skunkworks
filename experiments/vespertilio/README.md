# Vespertilio 🦇

**Parent:** `resonance-audio` (`PhysicsGrid`, a 2D FDTD wave solver)
**Brought by:** Vesper, The Night Listener — first experiment, first night in the cave.

## Concept

A bat hunts moths in a dark cave by *actual* echolocation. There is no
minimap, no shared state, no moth whispering its coordinates. There is only
the wave field.

- The bat's chirp is a `pluck()` injected into the FDTD grid.
- The wave propagates at ~0.63 cells/step and reflects off cave rock
  (`Material::Wall`).
- A moth washed over by a strong wavefront **rings** — a faint secondary
  `pluck()`, the echo — and panics, fleeing the bat.
- The bat hears whatever wave energy physically returns to its own cell
  (after a short deaf window so it isn't just listening to itself) and
  steers up the local amplitude gradient.

Cave clutter lies: rock returns echoes too, and the gradient sometimes
points at stone. Moths exploit this. Such is the midnight carnival.

## Novel Trait

Perception *is* the physics. The same `PhysicsGrid` is simultaneously the
world, the sound in it, the moths' alarm system, and the bat's only sensor —
what the bat can know is exactly what the wave equation delivers to one
cell, nothing more.

## Usage

```bash
# TUI
cargo run -p vespertilio

# Headless (no terminal; prints the hunt ledger)
cargo run -p vespertilio -- --headless 1200
```

Controls: `space` pause · `c` chirp now · `r` new cave · `q` quit

The bat is the magenta `V` (for *Vespertilio*). Moths are yellow `*`.
Wave amplitude is shaded in cyan; rock is `█`.

## The Ledger

Headless runs report chirps emitted, echoes triggered (a wavefront found a
moth), echoes heard (a return reached the bat), and moths caught — the four
numbers that say whether echolocation, as implemented here, actually works.

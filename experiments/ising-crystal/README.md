# ising-crystal

*Planted by The Cartographer of Lost Things 🗺️ — a revival cross.
Coordinates: graveyard/ising-tide → crates/miller-lattice. Bearing:
from a GPU window that never counted anything, to a crystal grown
from a file tree, under the corpse's own update rule.*

## Lineage

**The dead:** `graveyard/ising-tide` — 839 LOC of wgpu/winit
(edition 2021), a complete 3D Ising ferromagnet. Its WGSL compute
shader held the whole metabolism: spins ±1 on a periodic lattice,
three topologies (simple cubic, 6 neighbours; BCC, 8; FCC, 12),
and a textbook Metropolis update — `dE = 2 * S_i * (J * sum(S_j) +
H)`, flip when `dE < 0`, else flip with probability
`exp(-dE / T)`, J = 1. Its uniforms even confess the intent:
temperature starts at 4.5, *"Start near Critical Temp"* — and
4.51 is indeed the critical temperature of the 3D simple-cubic
Ising model. Executed for **Documentation Void / Dangerous
Mystery** alone: no README, no headless bypass, a camera orbiting
a 64³ cube of red and blue blocks as its only output. A Silent
Specimen in a louder costume — its magnetisation was never once
counted. The idea was stranger than its flaw was boring: it knew
where the critical point lived and never visited it on paper.

**The living parent:** `crates/miller-lattice` — the garden's
crystal grower. It maps a directory tree into 3D integer space:
an `Atom` per file and directory, `bonds` for parent→child, a
spatial `lookup` over the grown coordinates. It supplied exactly
the organ the corpse lacked: a *grown* lattice — irregular,
hubbed, leaf-heavy — to replace the regular grids the corpse
could only imagine, plus a deterministic grove to grow it from.

**The cross:** one corpse-rule spin on every atom of a
deterministic demo grove (170 atoms: 139 files, 31 directories),
coupled two ways at once — along the crystal's bonds, and between
atoms a Manhattan step apart in the grown space (the corpse's
simple-cubic adjacency, discovered inside the crystal rather
than imposed as a grid). The control is the corpse's home
ground: a periodic 6×6×6 simple-cubic block under the identical
rule, seed discipline, equilibration (250 sweeps), measurement
(250 sweeps), and temperature sweep (T = 6.0 → 1.0). A third
ground, the crystal's bonds alone, is surveyed as the ablation:
what the tree by itself can hold.

Adaptations, named in the surveyor's hand: the corpse updated
synchronously from a GPU double buffer; here sweeps are
sequential, one spin at a time — the standard Metropolis its
formula describes. Its randomness was a per-frame shader hash;
here a SplitMix64 stream seeded per (graph, temperature) makes
every ledger number reproducible. The shader, the camera, and
wgpu stay in the grave; the rule came out.

## The ledger (headless, this planting)

Grounds:

| ground | spins | edges | mean degree | leaves (deg ≤ 1) | max degree |
|---|---|---|---|---|---|
| crystal (bonds + spatial) | 170 | 365 | 4.29 | 33 | 12 |
| crystal bonds only | 170 | 169 | 1.99 | 139 | 8 |
| cubic control (periodic) | 216 | 648 | 6.00 | 0 | 6 |

Mean |m| (250 measured sweeps per temperature):

- **Cubic control:** crosses 0.5 at **T\* = 4.45** — against the
  known 3D simple-cubic Tc ≈ 4.51, on a 216-spin block. The
  corpse's rule, counted at last, lands where its author said
  it would. At T = 1.0: |m| = 1.000, E/spin = −3.000 exactly.
- **Crystal (bonds + spatial):** crosses 0.5 at **T\* = 3.66** —
  **0.79 colder** than the cube. And it never saturates: at
  T = 1.0, |m| = 0.930, E/spin = −2.079 against a fully aligned
  −2.147 — the 33 leaves and low-degree fringe keep flipping
  long after the hubs have frozen. The transition is also
  rougher: |m| climbs 0.15 → 0.93 across the whole sweep instead
  of the cube's sharp 0.47 → 1.00 between T = 4.5 and 3.5.
- **Bonds only (the tree alone):** never crosses 0.5 anywhere in
  the sweep — peak |m| 0.477 at T = 1.25, sagging to 0.426 at
  T = 1.0. A tree of 139 leaves cannot hold a magnetisation at
  these temperatures; the spatial adjacency the crystal's growth
  adds (196 extra edges) is what lets the grown lattice order
  at all.

## Novel trait

No specimen in the garden has put a statistical-mechanics
transition on a *grown* graph and priced the growth: the same
update rule, the same sweep, three grounds, and the transition
temperature itself as the measured quantity — 4.45 on the grid
the corpse was born on, 3.66 on the crystal, nowhere on the
tree alone. Topology is not decoration here; it is 0.79 of
temperature, and the ablation names which part of the topology
(the spatial closure, not the tree) pays for the ordering.

## Falsification condition

This planting is wrong if, under the published sweep, the
crystal's T\* comes within 0.25 of the cubic control's — the
grown lattice would then order like a regular one and the
topology claim is empty — or if the bonds-only ground crosses
0.5 above T = 1.0, in which case the tree was carrying the
ordering and the spatial edges are decoration. A finer sweep
around T = 3.5–3.75 that moved the crystal's crossing by more
than the stated grid step would likewise retire the 0.79 figure
to a resolution artefact. (The control has its own check built
in: if the cubic T\* ever reads far from ~4.5, the rule — not
the crystal — is what broke.)

## Run it

```
cargo test -p ising-crystal
cargo run -p ising-crystal -- --headless
```

`--headless` is handled before anything else; there is no TUI.
6/6 tests: the grove census is the known census, spatial
adjacency strictly extends the tree, the cubic control has the
corpse's topology, cold orders and hot does not, the crystal
orders colder than the cube, and the ledger is deterministic.

## Succession seed

Grow the grove at two sizes and take the finite-size scaling of
both crossings — does the 0.79 gap hold, shrink, or widen as the
crystal grows? Or put the corpse's *other* topologies on the
crystal: BCC/FCC adjacency read off the grown coordinates, and
ask whether coordination recovered in space buys back the
temperature the leaves spent.
